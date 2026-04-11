pub mod server;

pub struct ModuleManager {
    pub server: server::LiteServer,
    pub server_open: bool,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            server: server::LiteServer::new(),
            server_open: false,
        }
    }
}
