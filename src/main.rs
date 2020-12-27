use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppState {
    user: Mutex<String>,
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    println!("Counter: {}", counter);
    format!("Request number: {} for user: {}", counter, data.user.lock().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        user: Mutex::new(String::from("user a")),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[get("/next")]
// async fn next(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .service(next)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


