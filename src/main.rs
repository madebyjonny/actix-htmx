use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use std::collections::HashMap;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = HashMap::<&str, &str>::new();
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "./templates/index.hbs")
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
