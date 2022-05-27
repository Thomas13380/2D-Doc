use actix_web::{web, App, HttpServer};
mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/2ddoc", web::post().to(handlers::create_2ddoc))

    })
    .bind("0.0.0.0:4000")?
    .run()
    .await    
}




