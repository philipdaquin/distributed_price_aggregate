
use actix_web::{ route, HttpResponse, Result, web, HttpRequest};

use crate::{service::worker_service::WorkerService, models::cache_details::CacheDetails};


pub fn configure_analysis_service(cfg: &mut web::ServiceConfig) { 
    cfg
    .service(post_new_jobs);
}


#[route("/api/send", method = "POST")]
pub async fn post_new_jobs(data: web::Json<CacheDetails>) -> Result<HttpResponse> { 
    let cache_details = data.into_inner();
    // Send jobs to server 
    let _ = WorkerService::send_jobs(cache_details).await.unwrap();

    Ok(HttpResponse::Ok().finish())
}