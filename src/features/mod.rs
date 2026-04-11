pub mod server;
pub mod kasir;

pub struct ModuleManager {
    pub server: server::LiteServer,
    pub kasir: kasir::KasirModule,
    pub server_open: bool,
    pub kasir_open: bool,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            server: server::LiteServer::new(),
            kasir: kasir::KasirModule::new(),
            server_open: false,
            kasir_open: false,
        }
    }
}
