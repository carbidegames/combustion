extern crate combustion;

use combustion::runtime::RendererFactory as CRendererFactory;

pub struct RendererFactory {
}

impl CRendererFactory for RendererFactory {
}

pub fn factory() -> RendererFactory {
    RendererFactory {
    }
}
