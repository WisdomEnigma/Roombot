use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, Result};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use gpt_text::openai;
use regex::Regex;
use auth::Gatekeeper;
// use img2vec::vec_middleware;
use handlebars::Handlebars;
use core::panic;
use futures_util::stream::TryStreamExt;
use std::path::Path;
use std::{path::PathBuf, collections::HashMap, io::BufReader, fs::File};
use directories::UserDirs;
use rodio::OutputStream;
use pinata_ipfs::ipinata;
use music_stream::music;
use music_stream::music_blob;



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


#[derive(Deserialize)]
struct MusicStream{
    cover : String,
    artist : String,
    mfile : String,
    date : String,
    genre : String,
    composer: String,

    lyricst : String,
    studio : String,
    website : String,
    brand : String,
    royalty : String,
    ltbtc : String,

    lightnode : String,
    work : String,
    future : String,
    ownership : String,
    email : String,
}

#[derive(Deserialize, Debug)]

struct Authenicate{
    username : String, 
    email : String

}

// static variables 

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


#[get("/user/composer")]
async fn artist() -> impl Responder{

    NamedFile::open_async("./static/artists.html").await
}

// mut payload : web::Payload

#[post("/user/composer/newsong")]
async fn newsong_record(hbr : web::Data<Handlebars<'_>>,form : web::Form<MusicStream>) -> impl Responder{
    
    
    

        // let load = match payload.next().await{
        //     Some(data) => data,
        //     None => panic!("Error report"),
        // };

        // let res = match load{
        //     Ok(data) => data,
        //     Err(err) => panic!("Error report {:?}", err),
        // };


        // let res_value = match std::str::from_utf8(&res){
        //     Ok(value) => value,
        //     Err(err) => panic!("{:?}", err),
        // };

        let cover_img = &form.cover;

        let artists = &form.artist;

        let music_file = &form.mfile;

        

        let date = &form.date;

        let genre = &form.genre;

        let compose = &form.composer;

        let lyrics = &form.lyricst;

        let studio = &form.studio; 

        let website = &form.website;

        let endrosment = &form.brand;

        let royalty = &form.royalty; 

        let lightnode = &form.ltbtc;

        let lightnode_add = &form.lightnode;


        let work = &form.work;  

        let future = &form.future;

        let ownership = &form.ownership; 

        
        let email = &form.email; 


        if let Some(down_dir) = UserDirs::new(){

            if let Some(path) = down_dir.download_dir(){
                
                if !path.join(PathBuf::from(cover_img.to_owned())).exists(){

                    panic!("Error this file may be moved");
                }

                let mut art : Vec::<String> = Vec::<_>::new();
                art.push(artists.to_string());

                let mut earn : bool = false;
                let mut node : bool = false;
                let mut asset : bool = false;
                let mut fut : bool = false;
                let mut owner : bool = false;
                if royalty == "on" && lightnode == "on" && work == "on" && future == "on" && ownership == "on"{
                    earn = true;
                    node = true;
                    asset = true;
                    fut = true;
                    owner = true;
                }
                
                let mut record = music::new_beat(music_file.to_owned().to_string(), 
                art,cover_img.to_string(), 
                lightnode_add.to_string(), date.to_string(),lyrics.to_string(),
                studio.to_string(),genre.to_string(),compose.to_string(),
                website.to_string(), endrosment.to_string(),earn,node,asset,fut,owner,email.to_string());
                
                let client = match record.create_mongo_connection().await {

                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);

                let collection = match record.create_collection(db).await{
                    Ok(collect) => collect,
                    Err(e) => panic!("{:?}", e),
                };

                let dpath = path.join(cover_img.to_owned());
                
                let mut blob = ipinata::new_bolb_object(&dpath, ipinata::FileStatus::Pin);
                let pin_client = blob.pinta_client();

                let auth = pin_client.test_authentication().await;
                let content = blob.upload_content(pin_client, path.join(cover_img.to_owned()).display().to_string()).await;

                let mut cid : String = "".to_string(); 
                if let Ok(object) = content {
                    cid = object.ipfs_hash;
                }

                // let mut uplink_ob = uplink_config::new_uplink();
                let connect = music_blob::connect_with_uplink();

                if connect.error != std::ptr::null_mut(){

                    panic!("Error connection with uplink {:?}", connect.error);
                }

                
                
                println!("Content Indentifier {:?}", cid);
                return format!("Database created {:?}, pinata connection {:?}", collection, auth);
                
            }

        }


        format!("Error report")
    
}

#[get("/user/sociallink")]
async fn sociallink() -> impl Responder {

    NamedFile::open_async("./static/authlink.html").await
}

#[post("/user/sociallink/profile")]
async fn profile(form : web::Form<Authenicate>, hbr : web::Data<Handlebars<'_>> ) -> impl Responder{

    let username = &form.username;
    let email = &form.email;

   let auth_code = Gatekeeper::active_hash(&Gatekeeper::new_profile(email.to_string(), username.to_string()));

   let mut auth = Gatekeeper::Authenicate::new(auth_code.to_string(), username.to_string());

    let client = match Gatekeeper::mongodb_client().await {

        Ok(list) => list,
        Err(e) => panic!("{:?}", e),
    };

    let db = client.database(music::MUSIC_RECORD);
    let _ = auth.create_record(db).await;
    
    format!("Find User ")

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
            .service(artist)
            .service(newsong_record)
            .service(add_topic)
            .service(poetry)
            .service(history)
            .service(invoice)
            .service(configurations)
            .service(sociallink)
            .service(profile)
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



