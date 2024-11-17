use ggez::{event, graphics, Context, GameResult};
use ggez::input::keyboard::{self, KeyCode};

struct FlappyBird {
    bird_y: f32,
    bird_velocity: f32,
    gravity: f32,
}

impl FlappyBird {
    fn new() -> Self {
        FlappyBird {
            bird_y: 300.0,
            bird_velocity: 0.0,
            gravity: 0.5,
        }
    }
}

impl event::EventHandler for FlappyBird {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Apply gravity to the bird's velocity and update position
        self.bird_velocity += self.gravity;
        self.bird_y += self.bird_velocity;

        // Prevent the bird from falling below the screen
        if self.bird_y > 600.0 {
            self.bird_y = 600.0;
            self.bird_velocity = 0.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen
        graphics::clear(ctx, graphics::WHITE);

        // Draw the bird
        let bird = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [400.0, self.bird_y],
            20.0,
            2.0,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &bird, graphics::DrawParam::default())?;

        // Present the frame
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _repeat: bool) {
        if keycode == KeyCode::Space {
            // Make the bird jump
            self.bird_velocity = -10.0;
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("flappy_bird", "author_name")
        .build()
        .expect("Failed to build ggez context");

    let mut game = FlappyBird::new();
    event::run(&mut ctx, &mut event_loop, &mut game)
}

