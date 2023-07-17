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


#[actix_web::main]
 async fn main() -> std::io::Result<()>{

    HttpServer::new(|| {
            App::new()
            .service(index)
            .service(image_utopia)
            .service(image_learning)
            .service(translator)
            .service(register_user)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}
