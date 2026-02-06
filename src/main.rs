
use crate::{  error::error::AppError,  jobs::index::run_script};
pub mod error;
pub mod config;
pub mod state_models;
pub mod chain_config;
pub mod jobs;
pub mod tokens;


#[actix_web::main] 
async fn main() -> Result<(), AppError> {
    // Initialize logging first
    
    
    
    // Use join_all to wait for all workers
    let workers = vec![
        tokio::spawn(run_script(0)),
    ];
    
    // Wait for all workers (they should run forever unless error)
    for worker in workers {
        if let Err(e) = worker.await {
            tracing::error!("Worker failed: {:?}", e);
        }
    }
    
    tracing::info!("All workers stopped");
    Ok(())
}

