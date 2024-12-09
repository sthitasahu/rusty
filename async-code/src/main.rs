async fn fetch_data() -> String {
    "Data fetched".to_string()
}

async fn process_data(data: String) -> String {
    format!("Processed: {}", data)
}


#[tokio::main]
async fn main() {
    let data = fetch_data().await;
    let result = process_data(data).await;
    println!("{}", result);
}


