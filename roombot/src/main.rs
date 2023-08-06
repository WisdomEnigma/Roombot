use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, Result};
use actix_files::{NamedFile};
use serde::{Deserialize, Serialize};
use gpt_text ::{openai};
use regex::{Regex};




#[derive(Deserialize)]
struct TranslateFormData {

    query : String,
    call   : String,    
}



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
async fn word2word(form : web::Form<TranslateFormData>) -> HttpResponse{

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
        HttpResponse::BadRequest().body(format!("Queries have some bad words which are not acceptable by model"));
    }


    if !input.contains("translation") || !input.contains("translate"){

        println!("Sorry translation prompt will translate message for you ");
        HttpResponse::BadRequest().body(format!("Sorry translation prompt will translate message for you "));
    }


    let mut opencall : _ = openai::new(input.to_string(), "".to_string(), input.len().try_into().unwrap());
    
    let response =  match opencall.openai_text_wrapper(apikey.to_string()).await{

            Ok(resp) => resp,
            Err(e) => panic!("Error = {:?}", e),
    };

    HttpResponse::Ok().body(format!("{:?}, {:?}", input.to_string(), response))

}

#[get("/user/register")]
async fn register_user() -> impl Responder{

    
    NamedFile::open_async("./static/register.html").await
}

#[post("/user/register/verified")]
async fn register_face() -> impl Responder{

    format!("Image =")
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
async fn poetry(form : web::Form<TranslateFormData>) -> HttpResponse{

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
        HttpResponse::BadRequest().body(format!("Queries have some bad words which are not acceptable by model"));
    }


    let mut opencall : _ = openai::new(input.to_string(), "".to_string(), input.len().try_into().unwrap());
    
    let response =  match opencall.openai_openend(apikey.to_string()).await{

            Ok(resp) => resp,
            Err(e) => panic!("Error = {:?}", e),
    };

    HttpResponse::Ok().body(format!("{:?}, {:?}", input.to_string(), response))
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
            .service(register_face)
            .service(history)
            .service(invoice)
            .service(add_topic)
            .service(configurations)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}

