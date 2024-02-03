use super::{EntryRequestBody, GetErrorResponse, GetResponse, HttpError, PutResponse};
use crate::{
    rpc::{Entry, Key, Value},
    CacheClusterServer, HTTPServer,
};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/entry/{key}")]
pub(crate) async fn get(
    path: web::Path<(String,)>,
    cluster: web::Data<CacheClusterServer<HTTPServer>>,
) -> impl Responder {
    let key = &path.0;
    let key = Key { key: key.clone() };
    match cluster.network.lock().await.get_value(key).await {
        Ok(resp) => {
            if let Some(value) = resp.into_inner().value {
                HttpResponse::Ok().json(GetResponse { value: value.value })
            } else {
                HttpResponse::InternalServerError().json(GetErrorResponse {
                    error: HttpError::UnknownError,
                })
            }
        }
        _ => HttpResponse::NotFound().json(GetErrorResponse {
            error: HttpError::KeyNotFound,
        }),
    }
}

#[post("/entry")]
pub(crate) async fn save(
    path: web::Json<EntryRequestBody>,
    cluster: web::Data<CacheClusterServer<HTTPServer>>,
) -> impl Responder {
    let entry_req = path.into_inner();
    let entry = Entry {
        key: Some(Key { key: entry_req.key }),
        value: Some(Value {
            value: entry_req.value,
        }),
    };
    match cluster.network.lock().await.put_entry(entry).await {
        Ok(_) => HttpResponse::Ok().json(PutResponse { success: true }),
        // TODO: Provide more details about the errors
        _ => HttpResponse::BadRequest().json(GetErrorResponse {
            error: HttpError::BadRequest,
        }),
    }
}
