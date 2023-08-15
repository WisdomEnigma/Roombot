use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, Result};
use actix_files::{NamedFile};
use serde::{Deserialize, Serialize};
use gpt_text::{openai};
use regex::{Regex};
use img2vec::{vec_middleware};
use handlebars::{Handlebars};



#[derive(Deserialize)]
struct TranslateFormData {

    query : String,
    call   : String,    
}

#[derive(Serialize)]
struct ResponseTranslateForm{

    query : String,
    response : String,
}


#[derive(Serialize)]
struct ImageTemp{

    image : String,
}



#[get("/")]
async fn index() -> impl Responder {

    
    NamedFile::open_async("./static/index.html").await
}

#[get("/utopia")]
async fn image_utopia() -> impl Responder{

    NamedFile::open_async("./static/assets/utopia.jpg").await
}

#[get("/user_avatar")]
async fn avatari() -> impl Responder{

    NamedFile::open_async("/home/ali/Downloads/register_face.png").await
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
async fn word2word(form : web::Form<TranslateFormData>, hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

    let input : _ =  &form.query;
    let apikey : _ = &form.call; 

    let _ = vec!["eval", "echo", "system", "exec", "os", "kill", "script", "wget", "curl", 
                        "sudo", "cd", "chmod", "rm", "ls","cat", "rmdir", "grep", "tail", 
                        "mv", "chdir", "chown", "passwd", "unmask", "pwd", "mkdir", "clear", "cp",
                        "head", "whoami", "copy", "env"];

    let _ = vec!["nude", "porn", "xxx","sexy", "sex", "sexual"];

    let lines = input.lines();
    let bregex = Regex::new(r"\b(eval | echo | system |exec | os | kill | script | wget | curl | sudo | cd | chmod | rm | ls | cat | rmdir | grep | tail | mv | chdir | chown | passwd | unmask | pwd | mkdir | clear| cp | head | whoami | copy | env )").unwrap();
    let xregex = Regex::new(r"\b(nude | porn | xxx | sexy | sex | sexual )").unwrap();


    let mut take_action : bool = false;

    for words in lines{

        // for bad actors who invade system
        if bregex.is_match(words){
            take_action = true;
            break
        }

        // for bad boys
        if xregex.is_match(words){
            take_action = true;
            break
        }
    }

    if take_action{

        println!("Queries have some bad words which are not acceptable by model");
        
        HttpResponse::BadRequest().body(hbr.render("error", &ResponseTranslateForm{
            query : "".to_string(),
            response : "Beware there may be some bad words in a content re-structure your query.".to_string(),
        }).unwrap());
    }


    if !input.contains("translation") || !input.contains("translate"){

        HttpResponse::BadRequest().body(hbr.render("error", &ResponseTranslateForm{
            query : "".to_string(),
            response : "Beware there may be some bad words in a content re-structure your query.".to_string(),
        }).unwrap());
    }


    let mut opencall : _ = openai::new(input.to_string(), "".to_string(), input.len().try_into().unwrap());
    
    let responses =  match opencall.openai_text_wrapper(apikey.to_string()).await{

            Ok(resp) => format!("{:?}", resp),
            Err(e) => panic!("Error = {:?}", e),
    };

    HttpResponse::Ok().body(hbr.render("translate", &ResponseTranslateForm{
        query : input.to_string(),
        response : responses,
    }).unwrap())

}

#[get("/user/register")]
async fn register_user() -> impl Responder{

    
    NamedFile::open_async("./static/register.html").await
}

#[post("/user/register/verified")]
async fn register_face(hbr : web::Data<Handlebars<'_>>) -> impl Responder{

    let db : _ = vec_middleware::create_index();

    let  _ = match vec_middleware::register_face(db.await).await {

        Err(err) => panic!("Error : {:?}", err),
        Ok(_) => {},
    };

    HttpResponse::Ok().body(hbr.render("register", &ImageTemp{
        image : "/user_avata".to_string(),        
    }).unwrap())


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

#[post("/user/poetry/topics/{output}")]
async fn poetry(form : web::Form<TranslateFormData>, hbr : web::Data::<Handlebars<'_>>) -> HttpResponse{

    let input : _ =  &form.query;
    let apikey : _ = &form.call; 

    let _ = vec!["eval", "echo", "system", "exec", "os", "kill", "script", "wget", "curl", 
                        "sudo", "cd", "chmod", "rm", "ls","cat", "rmdir", "grep", "tail", 
                        "mv", "chdir", "chown", "passwd", "unmask", "pwd", "mkdir", "clear", "cp",
                        "head", "whoami", "copy", "env"];

    let _ = vec!["nude", "porn", "xxx","sexy", "sex", "sexual"];

    let lines = input.lines();
    let bregex = Regex::new(r"\b(eval | echo | system |exec | os | kill | script | wget | curl | sudo | cd | chmod | rm | ls | cat | rmdir | grep | tail | mv | chdir | chown | passwd | unmask | pwd | mkdir | clear| cp | head | whoami | copy | env )").unwrap();
    let xregex = Regex::new(r"\b(nude | porn | xxx | sexy | sex | sexual )").unwrap();


    let mut take_action : bool = false;

    for words in lines{

        // for bad actors who invade system
        if bregex.is_match(words){
            take_action = true;
            break
        }

        // for bad boys
        if xregex.is_match(words){
            take_action = true;
            break
        }
    }

    if take_action{

        HttpResponse::BadRequest().body(hbr.render("error", &ResponseTranslateForm{
            query : "".to_string(),
            response : "Beware there may be some bad words in a content re-structure your query.".to_string(),
        }).unwrap());
    }


    let mut opencall : _ = openai::new(input.to_string(), "".to_string(), input.len().try_into().unwrap());
    
    let responses =  match opencall.openai_openend(apikey.to_string()).await{

            Ok(resp) => format!("{:?}", resp),
            Err(e) => panic!("Error = {:?}", e),
    };

    HttpResponse::Ok().body(hbr.render("translate", &ResponseTranslateForm{
        query : input.to_string(),
        response : responses,
    }).unwrap())
}

#[get("/configurations")]
async fn configurations() -> impl Responder{

    
    NamedFile::open_async("./static/interactive.html").await
}

#[actix_web::main]
 async fn main() -> std::io::Result<()>{

    // create handlebar new object, direct towards template directory. This direct used as reference for direction purpose.
    let mut handlebars_obj = Handlebars::new();
    handlebars_obj
            .register_templates_directory(".html", "./static/templates")
            .unwrap();

    // server hold handlebar template directory object value.
    let handlebars_ref = web::Data::new(handlebars_obj);
    
    
    // now server supported templates. These templates are render application state when a query execute.
    HttpServer::new( move || {
            App::new()
            .app_data(handlebars_ref.clone())
            .service(index)
            .service(image_utopia)
            .service(image_learning)
            .service(translator)
            .service(word2word)
            .service(register_user)
            .service(register_face)
            .service(history)
            .service(invoice)
            .service(avatari)
            .service(add_topic)
            .service(configurations)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}

