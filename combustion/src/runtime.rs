use std::path::PathBuf;

/// A container for all data needed to start up a Combustion game instance, deferring loading of
/// assets to later.
pub struct GameRunStub {
    pub path: PathBuf
}

impl GameRunStub {
    pub fn at_path<P: Into<PathBuf>>(path: P) -> GameRunStub {
        GameRunStub {
            path: path.into()
        }
    }

    pub fn with_renderer<F: RendererFactory>(self, _name: &str, _factory: F) -> GameRunStub {
        self
    }
}

pub trait RendererFactory {
}

/// A representation of a running Combustion game instance.
pub struct GameRuntime {
}

impl GameRuntime {
    pub fn start(_stub: GameRunStub) -> GameRuntime {
        GameRuntime {
        }
    }

    pub fn wait_for_close(self) {
    }
}
