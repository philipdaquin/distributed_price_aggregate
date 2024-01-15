use worker_d::{error::Result, server::WorkerServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u32>().ok())
        .unwrap_or(4000);

    let client = WorkerServer::new(port)
        .run_server()
        .await
        .map_err(Into::into);
    
    return client
}