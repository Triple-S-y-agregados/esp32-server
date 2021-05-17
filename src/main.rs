mod endpoints;

use actix_web::{App, HttpServer};

use endpoints::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(voltage)
            .service(records)
            .service(get_records)
            .service(clean)
    })
    .bind("127.0.0.1:44388")?
    .run()
    .await
}
