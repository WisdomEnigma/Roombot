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


#[get("/translation")]
async fn translator() -> impl Responder {

    NamedFile::open_async("./static/translate.html").await
}

#[get("/user/register")]

async fn register_user() -> impl Responder{

    NamedFile::open_async("./static/register.html").await
}

#[get("/user/history")]
async fn history() -> impl Responder {

    NamedFile::open_async("./static/history.html").await
}

#[get("/user/invoice")]
async fn invoice() -> impl Responder {

    NamedFile::open_async("./static/invoice.html").await
}

#[get("/user/poetry/topics")]
async fn add_topic() -> impl Responder{

    NamedFile::open_async("./static/poetry.html").await
}

#[actix_web::main]
 async fn main() -> std::io::Result<()>{

    HttpServer::new(|| {
            App::new()
            .service(index)
            .service(image_utopia)
            .service(image_learning)
            .service(translator)
            .service(register_user)
            .service(history)
            .service(invoice)
            .service(add_topic)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}
