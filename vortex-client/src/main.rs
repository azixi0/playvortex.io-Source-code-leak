// -- leaked by @azixi0 on github
use vortex_client_recovered::bootstrap::BootstrapConfig;

fn main() {
    let config = BootstrapConfig::from_environment();
    println!("Vortex v{} booting", vortex_engine::RECOVERED_VERSION);
    println!("backend stage: {}", config.backend_stage);
}
