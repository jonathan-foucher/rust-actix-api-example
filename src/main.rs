mod routers;
mod models;
use routers::http_server::start_server;

fn main() -> std::io::Result<()> {
    start_server()
}
