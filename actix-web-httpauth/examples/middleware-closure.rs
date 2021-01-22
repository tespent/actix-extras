use actix_web::{middleware, web, App, HttpServer};

use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::extractors::BasicAuth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::with_fn(|req, _credentials: BasicAuth| async { Ok(req) });
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .service(web::resource("/").to(|| async { "Test\r\n" }))
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}
