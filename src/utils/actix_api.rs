use crate::{rpc::Key, CacheClusterServer, HTTPServer};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/get/{key}")]
pub(crate) async fn cluster_get(
    path: web::Path<(String,)>,
    cluster_data: web::Data<CacheClusterServer<HTTPServer>>,
) -> impl Responder {
    let key = &path.0;
    let key = Key { key: key.clone() };
    match cluster_data.network.lock().await.get_value(key).await {
        // TODO: Handle the response
        _ => {}
    }

    HttpResponse::Ok()
}
