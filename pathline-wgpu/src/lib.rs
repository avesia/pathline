use pathline::context::Renderer;

struct WgpuState {
    device: wgpu::Device,
}

impl WgpuState {
    fn new() -> Self {
        todo!()
    }
}

pub struct WgpuRenderer {
    state: WgpuState,
}

impl WgpuRenderer {
    fn new() -> Self {
        WgpuRenderer {
            state: WgpuState::new()
        }
    }
}

impl Renderer for WgpuRenderer {
    fn clear_color() {
        
    }
}

pub fn wgpu_surface() -> WgpuRenderer {
    WgpuRenderer::new()
}
