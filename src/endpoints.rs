use actix_web::{post, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use database_lib::{create_record};

#[derive(Deserialize, Serialize)]
struct NewRecord {
    voltage: i32
}

#[post("/voltage")]
async fn voltage(record: web::Json<NewRecord>) -> impl Responder {
    create_record(&record.voltage);

    HttpResponse::Ok()
}
