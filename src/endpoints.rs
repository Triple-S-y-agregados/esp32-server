use actix_web::{get, post, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use database_lib::{create_record, get_all_records};

#[derive(Deserialize, Serialize)]
struct NewRecord {
    voltage: i32
}

#[derive(Deserialize, Serialize)]
struct Record {
    id: i32,
    timestamp: String,
    voltage: i32
}

#[post("/voltage")]
async fn voltage(record: web::Json<NewRecord>) -> impl Responder {
    create_record(&record.voltage);

    HttpResponse::Ok()
}

#[get("/records")]
async fn records() -> impl Responder {
    let records = get_all_records();
    let mut record_list: Vec<Record> = Vec::new();

    for record in &records {
        record_list
            .push(Record { 
                id: record.id,
                timestamp: record.timestamp.clone(),
                voltage: record.voltage 
            });
    }

    web::Json(record_list)
}
