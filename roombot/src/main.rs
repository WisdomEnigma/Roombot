use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, Result};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use gpt_text::openai;
use regex::Regex;
use img2vec::vec_middleware;
use handlebars::Handlebars;
use std::{path::PathBuf, collections::HashMap, io::BufReader};
use std::fs::File;
use directories::UserDirs;
use rodio::{Decoder,OutputStream};



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
struct Authorize{

    compress : String,
}


#[derive(Serialize)]
struct ImageTemp{

    image : String,
}

#[derive(Serialize)]
struct AudioSearchResults{

    audioname : String,
    isplay : bool,
}

#[derive(Serialize)]
struct AudioSearchError{

    error : String,
}

#[derive(Deserialize)]
struct SearchPlaylist{

    songname : String,
}

static mut AUDIO : Vec<HashMap<String, bool>> = Vec::new();




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

// #[get("/user/register")]
// async fn register_user() -> impl Responder{
   
//     NamedFile::open_async("./static/register.html").await
// }

// #[post("/user/register/verified")]
// async fn register_face(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

//     let db : _ = vec_middleware::create_index();

//     let  _ = match vec_middleware::register_face(db.await).await {

//         Err(err) => panic!("Error : {:?}", err),
//         Ok(_) => {},
//     };

//     HttpResponse::Ok().body(hbr.render("register", &ImageTemp{
//         image : "/user_avatar".to_string(),        
//     }).unwrap())


// }

// #[get("/user/login")]
// async fn login() -> impl Responder{
   
//     NamedFile::open_async("./static/login.html").await
// }


// #[post("/user/login/verified")]
// async fn login_account(hbr : web::Data<Handlebars<'_>>) -> impl Responder{

//     let db : _ = vec_middleware::create_index();

//     let value = vec_middleware::unlock_account(db.await).await;

//     format!("output = {:?}", value)

//     //HttpResponse::Ok().body(hbr.render("login", &Authorize{compress : value}).unwrap())

// }


#[get("/user/my/playlist")]
async fn playlist() -> impl Responder{

    NamedFile::open_async("./static/music.html").await
}

#[post("/user/my/playlist/{search}")]
async fn search_playlist(form : web::Form<SearchPlaylist>, hbr : web::Data<Handlebars<'_>>) -> HttpResponse {

    let query : _ = &form.songname;
    let audiomp3 = query.to_owned() + &".mp3";
    let audiowav = query.to_owned() + &".wav";
    let mut source : HashMap<String, bool> = HashMap::new();

    

    if let Some(dir) = UserDirs::new() {
        
        if let Some(file) = dir.audio_dir(){
            
            let mut file_ext = "".to_string();

            if PathBuf::from(&file.join(audiomp3.to_owned())).exists(){

                file_ext = PathBuf::from(&file.join(audiomp3)).display().to_string();

                let audio = file_ext.clone();
                
                source.insert(file_ext, true);

                set_audio(source);

                let datablock = get_audio();

                let object = match datablock.get_key_value(&audio){

                        Some(data) => {data},
                        None => {
                                    panic!("Error No Content in your playlist")
                        },
                };
                
                    let mut flag_audio : bool = false;

                    if object.1 != &true{
                                flag_audio = false;
                    }else{
                                flag_audio = true;
                    }

                    return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
                        audioname: format!("{:?}", object.0), 
                        isplay: flag_audio}
                    ).unwrap());
                
                               
            }else if PathBuf::from(&file.join(audiowav.to_owned())).exists(){

                file_ext = PathBuf::from(&file.join(audiowav)).display().to_string();

                let file = file_ext.clone();
                source.insert(file_ext, true);

                set_audio(source);

                let datablock = get_audio();

                let object = match datablock.get_key_value(&file){

                        Some(data) => {data},
                        None => {
                                    panic!("Error No Content in your playlist")
                        },
                };
                
                    let mut flag_audio : bool = false;

                    if object.1 != &true{
                                flag_audio = false;
                    }else{
                                flag_audio = true;
                    }

                    return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
                        audioname: format!("{:?}", object.0), 
                        isplay: flag_audio}
                    ).unwrap());
            
            }else{

                panic!("Format is not supported");
            }

            
        }
    }

    HttpResponse::Ok().body(hbr.render("music_error", &AudioSearchError{
        error : "This song not available in the content".to_string(),
    }).unwrap())


    
}

#[post("/user/my/playlist/{search}/play")]
async fn play_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = get_audio();
    for i in audio.keys(){
        
        let file = BufReader::new(File::open(i).unwrap());
        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.play();
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60*5));

        return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
            audioname: i.to_string(), 
            isplay: true}
        ).unwrap());
    }

    HttpResponse::Ok().body(hbr.render("music_error", &AudioSearchError{
        error : "Song is already playing".to_string(),
    }).unwrap())
    
}

#[post("/user/my/playlist/{search}/paused")]
async fn paused_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = get_audio();
    for i in audio.keys(){
        
        let file = BufReader::new(File::open(i).unwrap());
        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.pause();
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60*5));

        return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
            audioname: i.to_string(), 
            isplay: true}
        ).unwrap());
    }

    HttpResponse::Ok().body(hbr.render("music_error", &AudioSearchError{
        error : "Song is already playing".to_string(),
    }).unwrap())
    
}

#[post("/user/my/playlist/{search}/stop")]
async fn stop_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = get_audio();
    for i in audio.keys(){
        
        let file = BufReader::new(File::open(i).unwrap());
        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.stop();
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60*3));
        return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
            audioname: i.to_string(), 
            isplay: true}
        ).unwrap());
    }

    HttpResponse::Ok().body(hbr.render("music_error", &AudioSearchError{
        error : "Song had stopped".to_string(),
    }).unwrap())
}

#[post("/user/my/playlist/{search}/stepforward")]
async fn stepforward_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = get_audio();
    for i in audio.keys(){
        
        let file = BufReader::new(File::open(i).unwrap());

        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.skip_one();
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60*3));
        return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
            audioname: i.to_string(), 
            isplay: true}
        ).unwrap());
    }

    HttpResponse::Ok().body(hbr.render("music_error", &AudioSearchError{
        error : "No Song left".to_string(),
    }).unwrap())
}



#[post("/user/my/playlist/{search}/fastforward")]
async fn fastforward_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = get_audio();
    for i in audio.keys(){
        
        let file = BufReader::new(File::open(i).unwrap());

        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.set_speed(2.0);
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60*3));
        return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
            audioname: i.to_string(), 
            isplay: true}
        ).unwrap());
    }

    HttpResponse::Ok().body(hbr.render("music_error", &AudioSearchError{
        error : "No Song left".to_string(),
    }).unwrap())
}

#[post("/user/my/playlist/{search}/fastbackward")]
async fn fastbackward_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = get_audio();
    for i in audio.keys(){
        
        let file = BufReader::new(File::open(i).unwrap());

        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.set_speed(0.7);
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60*3));
        return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
            audioname: i.to_string(), 
            isplay: true}
        ).unwrap());
    }

    HttpResponse::Ok().body(hbr.render("music_error", &AudioSearchError{
        error : "No Song left".to_string(),
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
            .service(image_utopia)
            .service(image_learning)
            .service(avatari)
            .service(index)
            .service(translator)
            .service(word2word)
            .service(playlist)
            .service(search_playlist)
            .service(play_audio)
            .service(stop_audio)
            .service(stepforward_audio)
            .service(fastforward_audio)
            .service(fastbackward_audio)
            .service(paused_audio)
            .service(add_topic)
            .service(poetry)
            .service(history)
            .service(invoice)
            .service(configurations)
            // .service(register_user)
            // .service(register_face)
            // .service(login)
            // .service(login_account)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}



fn set_audio(source : HashMap<String, bool>){

    unsafe{
        AUDIO.push(source);
    }
}


fn get_audio() -> HashMap<String, bool> {

    let mut value : HashMap<String, bool> = HashMap::new();
    unsafe{
        

        for i in 0..AUDIO.len(){

            let key = match AUDIO.get(i){
               Some(k) => k,
               None => panic!("Error reporting"), 
            };

            value = key.clone();
            
        }
    }

    value
}