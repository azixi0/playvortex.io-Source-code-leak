// -- leaked by @azixi0 on github
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderBackend { Noop, Vulkan, Metal, Dx12, Gl, BrowserWebGpu }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapConfig {
    pub backend_stage: u8,
    pub requested_backend: Option<String>,
    pub log_filter: String,
}

impl BootstrapConfig {
    pub fn from_environment() -> Self {
        Self {
            backend_stage: env::var("VORTEX_BACKEND_STAGE").ok().and_then(|value| value.parse().ok()).unwrap_or(0),
            requested_backend: env::var("WGPU_BACKEND").ok(),
            log_filter: "warn,vortex_server=info,vortex_engine=info".to_owned(),
        }
    }

    pub fn fallback_order(&self) -> Vec<RenderBackend> {
        match self.backend_stage {
            0 => vec![RenderBackend::Dx12, RenderBackend::Vulkan, RenderBackend::Gl],
            1 => vec![RenderBackend::Vulkan, RenderBackend::Gl],
            _ => vec![RenderBackend::Gl],
        }
    }
}

pub const CRASH_LOG: &str = "crash.log";
pub const RENDER_MARKER: &str = "render_init.marker";
pub const APP_LOG: &str = "logs/vortex.log";

