use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    println!("Received request at /{}", name);
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
