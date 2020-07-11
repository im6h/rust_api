pub mod schema;
pub mod db_connection;
pub mod models;
pub mod api;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;

extern crate actix;
extern crate actix_web;

use actix_web::{App, HttpServer, web};

fn main() {
    let sys = actix::System::new("mystore");

    HttpServer::new(
    || App::new()
        .service(
            web::resource("/products")
                .route(web::get().to_async(api::products::index))
                .route(web::post().to_async(api::products::create))
        )
        .service(
            web::resource("/products/{id}")
                .route(web::get().to_async(api::products::show))
                .route(web::delete().to_async(api::products::destroy))
                .route(web::patch().to_async(api::products::update))
        )
        .service(
            web::resource("/links")
                .route(web::get().to_async(api::links::index))
                .route(web::post().to_async(api::links::create))
        )
        .service(
            web::resource("links/{id}")
            .route(web::get().to_async(api::links::show))
            .route(web::delete().to_async(api::links::delete))
            .route(web::patch().to_async(api::links::update))
        )
    )
    .bind("127.0.0.1:8088").unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}