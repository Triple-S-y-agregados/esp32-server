mod endpoints;

use actix_web::{App, HttpServer};

use endpoints::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(voltage)
            .service(records)
            .service(clean)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
