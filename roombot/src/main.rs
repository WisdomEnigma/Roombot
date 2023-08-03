use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use actix_files::{NamedFile};
use serde::{Deserialize, Serialize};
use gpt_text ::{openai};


#[derive(Deserialize)]
struct TranslateFormData {

    query : String,
    
}

const secret : String = "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".to_string();

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

#[post("/translation/user/{output}")]
async fn word2word(form : web::Form<TranslateFormData>) -> impl Responder{

    let input : _ =  &form.query;

    let mut opencall : _ = openai::new(input.to_string(), "".to_string(), input.len().try_into().unwrap());
    
    let response =  match opencall.openai_text_wrapper(secret).await{

        Ok(resp) => resp,
        Err(e) => panic!("Error = {:?}", e),
    };

    format!("Output =  {:?}", response)

    

    
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

#[get("/configurations")]
async fn configurations() -> impl Responder{

    
    NamedFile::open_async("./static/interactive.html").await
}

#[actix_web::main]
 async fn main() -> std::io::Result<()>{

    HttpServer::new(|| {
            App::new()
            .service(index)
            .service(image_utopia)
            .service(image_learning)
            .service(translator)
            .service(word2word)
            .service(register_user)
            .service(history)
            .service(invoice)
            .service(add_topic)
            .service(configurations)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}

