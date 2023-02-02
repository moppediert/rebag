use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("localhost", 8080))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    let mut body = "".to_string();
    let mut topics = read_bags(env::current_dir().unwrap().as_path());
    for (k, v) in topics.iter_mut() {
        body = format!("{}Bag: {}", body, k);
        v.sort();
        for topic in v {
            body = format!("{}\n{}", body, topic);
        }
        body = format!("{}\n\n-------------------------------\n\n", body);
    }
    HttpResponse::Ok().body(body)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}