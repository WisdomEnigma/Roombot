use actix_web::{get, web, App, HttpServer,HttpRequest, Responder, Result};
use actix_files::{NamedFile};

#[get("/")]
async fn index() -> impl Responder {

    NamedFile::open_async("./static/index.html").await
}

#[get("/utopia")]
async fn image_utopia() -> impl Responder{

    NamedFile::open_async("./static/assets/utopia.jpg").await
}

#[get("/futuristic")]
async fn image_learning() -> impl Responder{

    NamedFile::open_async("./static/assets/translation.png").await
}


#[actix_web::main]
 async fn main() -> std::io::Result<()>{

    HttpServer::new(|| {
            App::new()
            .service(index)
            .service(image_utopia)
            .service(image_learning)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}
