use actix_web::{get, post, delete, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use database_lib::{create_record, get_all_records, clean as database_clean, get_last_records};

#[derive(Deserialize, Serialize)]
struct NewRecord {
    voltage: f32
}

#[derive(Deserialize, Serialize)]
struct Record {
    id: i32,
    timestamp: String,
    voltage: f32
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

#[get("/records/{n}")]
async fn get_records(web::Path(n): web::Path<i64>) -> impl Responder {
    let other_records = get_last_records(n);

    let mut record_list: Vec<Record> = Vec::new();

    for record in &other_records {
        record_list
            .push(Record { 
                id: record.id,
                timestamp: record.timestamp.clone(),
                voltage: record.voltage 
            });
    }

    web::Json(record_list)
}

#[delete("/records")]
async fn clean() -> impl Responder {
    match database_clean() {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::NotFound()
    }
}
