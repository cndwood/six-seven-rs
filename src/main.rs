mod app;
mod error;
mod input;
mod rendering;
mod scene;
mod strings;
mod timer;

pub use error::Result;

use app::App;

fn main() -> Result<()> {
    App::new()?.run()
}
