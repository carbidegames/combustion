extern crate combustion;
extern crate combustion_glium;

use combustion::runtime::{GameRunStub, GameRuntime};

fn main() {
    // Set up the game's available modules
    let stub = GameRunStub::at_path("../")
        .with_renderer("Glium", combustion_glium::factory());

    // Run the game
    let runtime = GameRuntime::start(stub);
    runtime.wait_for_close();
}
