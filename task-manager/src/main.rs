use axum::{
    extract::Path,
    routing::{get, post, delete, put},
    Router, Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};
use tokio::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTask {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTask {
    title: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

type TaskStore = Arc<Mutex<HashMap<usize, Task>>>;

const STORAGE_FILE: &str = "tasks.json";

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(load_tasks()));

    let app = Router::new()
        .route("/tasks", get(list_tasks))
        .route("/tasks/:id", get(get_task))
        .route("/tasks", post(create_task))
        .route("/tasks/:id", put(update_task))
        .route("/tasks/:id", delete(delete_task))
        .route("/tasks/:id/toggle", post(toggle_task))
        .with_state(store);

    println!("Server running on http://localhost:3000");
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn load_tasks() -> HashMap<usize, Task> {
    match fs::read_to_string(STORAGE_FILE) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

fn save_tasks(tasks: &HashMap<usize, Task>) {
    if let Ok(json) = serde_json::to_string_pretty(tasks) {
        fs::write(STORAGE_FILE, json).unwrap_or_else(|e| eprintln!("Failed to save tasks: {}", e));
    }
}

async fn list_tasks(
    store: axum::extract::State<TaskStore>,
) -> Json<ApiResponse<Vec<Task>>> {
    let tasks = store.lock().await;
    let tasks_vec: Vec<Task> = tasks.values().cloned().collect();
    
    Json(ApiResponse {
        success: true,
        message: format!("Successfully retrieved {} tasks", tasks_vec.len()),
        data: Some(tasks_vec),
    })
}

async fn get_task(
    store: axum::extract::State<TaskStore>,
    Path(id): Path<usize>,
) -> Json<ApiResponse<Task>> {
    let tasks = store.lock().await;
    
    match tasks.get(&id) {
        Some(task) => Json(ApiResponse {
            success: true,
            message: format!("Successfully retrieved task {}", id),
            data: Some(task.clone()),
        }),
        None => Json(ApiResponse {
            success: false,
            message: format!("Task with id {} not found", id),
            data: None,
        }),
    }
}

async fn create_task(
    store: axum::extract::State<TaskStore>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<ApiResponse<Task>>) {
    let mut tasks = store.lock().await;
    
    let id = tasks.len() + 1;
    let task = Task {
        id,
        title: payload.title,
        completed: false,
    };
    
    tasks.insert(id, task.clone());
    save_tasks(&tasks);
    
    (StatusCode::CREATED, Json(ApiResponse {
        success: true,
        message: format!("Task created successfully with id {}", id),
        data: Some(task),
    }))
}

async fn update_task(
    store: axum::extract::State<TaskStore>,
    Path(id): Path<usize>,
    Json(payload): Json<UpdateTask>,
) -> Json<ApiResponse<Task>> {
    let mut tasks = store.lock().await;
    
    if let Some(task) = tasks.get_mut(&id) {
        if let Some(title) = payload.title {
            task.title = title;
        }
        if let Some(completed) = payload.completed {
            task.completed = completed;
        }
        let updated_task = task.clone();
        save_tasks(&tasks);
        
        Json(ApiResponse {
            success: true,
            message: format!("Task {} updated successfully", id),
            data: Some(updated_task),
        })
    } else {
        Json(ApiResponse {
            success: false,
            message: format!("Task with id {} not found", id),
            data: None,
        })
    }
}

async fn delete_task(
    store: axum::extract::State<TaskStore>,
    Path(id): Path<usize>,
) -> Json<ApiResponse<()>> {
    let mut tasks = store.lock().await;
    
    if tasks.remove(&id).is_some() {
        save_tasks(&tasks);
        Json(ApiResponse {
            success: true,
            message: format!("Task {} deleted successfully", id),
            data: None,
        })
    } else {
        Json(ApiResponse {
            success: false,
            message: format!("Task with id {} not found", id),
            data: None,
        })
    }
}

async fn toggle_task(
    store: axum::extract::State<TaskStore>,
    Path(id): Path<usize>,
) -> Json<ApiResponse<Task>> {
    let mut tasks = store.lock().await;
    
    if let Some(task) = tasks.get_mut(&id) {
        task.completed = !task.completed;
        let updated_task = task.clone();
        save_tasks(&tasks);
        
        Json(ApiResponse {
            success: true,
            message: format!("Task {} toggled successfully. New status: {}", id, updated_task.completed),
            data: Some(updated_task),
        })
    } else {
        Json(ApiResponse {
            success: false,
            message: format!("Task with id {} not found", id),
            data: None,
        })
    }
}