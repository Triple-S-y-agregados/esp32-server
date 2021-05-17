mod endpoints;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use endpoints::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_header()
              .allow_any_method()
              .max_age(3600);

        App::new()
            .wrap(cors)
            .service(voltage)
            .service(records)
            .service(get_records)
            .service(clean)
    })
    .bind("0.0.0.0:44388")?
    .run()
    .await
}
