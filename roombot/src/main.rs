use actix_files::NamedFile;
#[warn(non_camel_case_types)]
#[warn(unused_imports)]
#[warn(unused_assignments)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use auth::gatekeeper;
use gpt_text::openai;
use serde::{Deserialize, Serialize};
// use img2vec::vec_middleware;
use core::panic;
use handlebars::Handlebars;
//use futures_util::stream::TryStreamExt;
//use std::path::Path;
use directories::UserDirs;
use l2net::lightnode_net::{self, INodeless};
use music_stream::{music, pinata_content};
use once_cell::sync::OnceCell;
use pinata_ipfs::ipinata;
use rodio::OutputStream;
use std::{fs::File, io::BufReader, path::PathBuf, collections::HashMap};

// private structures

#[derive(Deserialize)]
struct TranslateFormData {
    query: String,
    call: String,
}

#[derive(Serialize)]
struct ResponseTranslateForm {
    query: String,
    response: String,
}

#[derive(Serialize)]
struct Authorize {
    compress: String,
}

#[derive(Serialize)]
struct ImageTemp {
    image: String,
}

#[derive(Serialize)]
struct AudioSearchResults {
    audioname: String,
    isplay: bool,
}

#[derive(Serialize)]

struct Nftmint {
    session: String,
    song: String,
    cid_image: String,
    cid_music: String,
    amount: String,

}

#[derive(Deserialize)]
struct SearchPlaylist {
    songname: String,
}

#[derive(Deserialize)]
struct Commenting{

    icomment : String,
}

#[derive(Deserialize)]
struct MusicStream {
    cover: String,
    artist: String,
    mfile: String,
    date: String,
    genre: String,
    composer: String,

    lyricst: String,
    studio: String,
    website: String,
    brand: String,
    royalty: String,
    ltbtc: String,

    lightnode: String,
    work: String,
    future: String,
    ownership: String,
    email: String,
}

#[derive(Deserialize, Debug)]

struct Authenicate {
    username: String,
    email: String,
}

#[derive(Serialize)]

struct Homepage;

#[derive(Serialize)]
struct History;

#[derive(Serialize)]
struct RequestError;

#[derive(Serialize)]
struct SongEngine {
    pmusic_artist: String,
    pmusic_compose: String,
    pmusic_lyric: String,
    pmusic_genre: String,
    pnumic_production: String,
    pmusic_ilink: String,
    pmusic_mlink: String,
    session: String,
    name: String,
    favourite: bool,
    favourite_count: i64,
    played: i64,
    emotion: pinata_content::Emotionfilter,
    comment : String,
    comment_like_count : i64,
    comment_likes : bool,
    user_comments : i64,
}

// static variables

static mut ME: u64 = 0;
static mut LIKES: i64 = 0;
static mut COLORED: bool = false;
static mut PLAY: i64 = 0;
static mut USERCOMMENTS :  i64 = 0;
static GLOBAL_SONG: OnceCell<String> = OnceCell::new();
static MY_COMMENT : OnceCell<String> = OnceCell::new();

// routes
#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}

#[get("/utopia")]
async fn image_utopia() -> impl Responder {
    NamedFile::open_async("./static/assets/utopia.jpg").await
}

#[get("/user_avatar")]
async fn avatari() -> impl Responder {
    NamedFile::open_async("/home/ali/Downloads/register_face.png").await
}

#[get("/futuristic")]
async fn image_learning() -> impl Responder {
    NamedFile::open_async("./static/assets/translation.png").await
}

#[get("/translation")]
async fn translator() -> impl Responder {
    NamedFile::open_async("./static/translate.html").await
}

#[post("/translation/user/{output}")]
async fn word2word(
    form: web::Form<TranslateFormData>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // parse input values
    let input: _ = &form.query;
    let apikey: _ = &form.call;

    let action = openai::validator(input.to_string());

    // check whether any bad word exist in a text
    if let Ok(take_action) = action {
        if take_action {
            println!("Check your text there may be something which is not acceptable");
            HttpResponse::BadRequest().body(hbr.render("error", &RequestError {}).unwrap());
        }
    }

    // validate keyword transalation or translate
    if !input.contains("translation")
        || !input.contains("translate") && !input.contains("Translation")
        || !input.contains("Translate")
    {
        println!("Trigger word is not present 'Translate '  ");
        HttpResponse::BadRequest().body(hbr.render("error", &RequestError {}).unwrap());
    }

    // engage openai call
    let mut opencall: _ = openai::new(
        input.to_string(),
        "".to_string(),
        input.len().try_into().unwrap(),
    );

    let responses = match opencall.openai_text_wrapper(apikey.to_string()).await {
        Ok(resp) => format!("{:?}", resp),
        Err(e) => panic!("Error = {:?}", e),
    };

    HttpResponse::Ok().body(
        hbr.render(
            "translate",
            &ResponseTranslateForm {
                query: input.to_string(),
                response: responses,
            },
        )
        .unwrap(),
    )
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
async fn playlist() -> impl Responder {
    NamedFile::open_async("./static/music.html").await
}

// This feature allow user to listen music on web without any media player.
// Suppose you're a taylor big fan and you wanna listen "The Moment I Knew"
// all you have type a name of a song and play it for you.

#[post("/user/my/playlist/{search}")]
async fn search_playlist(
    form: web::Form<SearchPlaylist>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // parse input values
    let query: _ = &form.songname;
    let audiomp3 = query.to_owned() + &".mp3";
    let audiowav = query.to_owned() + &".wav";

    // single source for storing audio search results ; in upcoming version multiple songs will be searched
    let mut source: HashMap<String, bool> = HashMap::new();

    // get file from directory and if found store in the source
    if let Some(dir) = UserDirs::new() {
        if let Some(file) = dir.audio_dir() {
            let mut file_ext = "".to_string();

            // whether audio mp3 or wav

            if PathBuf::from(&file.join(audiomp3.to_owned())).exists() {
                file_ext = PathBuf::from(&file.join(audiomp3)).display().to_string();

                let audio = file_ext.clone();

                source.insert(file_ext, true);

                music::set_audio(source);

                let datablock = music::get_audio();

                let object = match datablock.get_key_value(&audio) {
                    Some(data) => data,
                    None => {
                        panic!("Error No Content in your playlist")
                    }
                };

                let mut flag_audio: bool = false;

                if object.1 != &true && flag_audio == false {
                    flag_audio = false;
                } else {
                    flag_audio = true;
                }

                return HttpResponse::Ok().body(
                    hbr.render(
                        "music",
                        &AudioSearchResults {
                            audioname: format!("{:?}", object.0),
                            isplay: flag_audio,
                        },
                    )
                    .unwrap(),
                );
            } else if PathBuf::from(&file.join(audiowav.to_owned())).exists() {
                file_ext = PathBuf::from(&file.join(audiowav)).display().to_string();

                let file = file_ext.clone();
                source.insert(file_ext, true);

                music::set_audio(source);

                let datablock = music::get_audio();

                let object = match datablock.get_key_value(&file) {
                    Some(data) => data,
                    None => {
                        panic!("Error No Content in your playlist")
                    }
                };

                let mut flag_audio: bool = false;

                if object.1 != &true && flag_audio == false {
                    flag_audio = false;
                } else {
                    flag_audio = true;
                }

                return HttpResponse::Ok().body(
                    hbr.render(
                        "music",
                        &AudioSearchResults {
                            audioname: format!("{:?}", object.0),
                            isplay: flag_audio,
                        },
                    )
                    .unwrap(),
                );
            } else {
                panic!("Format is not supported");
            }
        }
    }

    println!("Song you wanan listen unforuentely not available in your directory. Please visit our song collection link where your favourite song might be ready for you. ");
    HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError {}).unwrap())
}

// web music player player your favourite song "The Moment I Knew". So phenomenal

#[post("/user/my/playlist/{search}/play")]
async fn play_audio(hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    
    // open audio file, with how many times audio will be played.
    // run the audio file ; if exist
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = music::get_audio();
    
    for i in audio.keys() {
        let file = BufReader::new(File::open(i).unwrap());
        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.play();
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60 * 5));

        return HttpResponse::Ok().body(
            hbr.render(
                "music",
                &AudioSearchResults {
                    audioname: i.to_string(),
                    isplay: true,
                },
            )
            .unwrap(),
        );
    }

    println!("Please check your volume or other peripheral devices attached to your devices");
    HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError {}).unwrap())
}

// #[post("/user/my/playlist/{search}/paused")]
// async fn paused_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let audio = get_audio();
//     for i in audio.keys(){

//         let file = BufReader::new(File::open(i).unwrap());
//         let clip = stream_handle.play_once(file).unwrap();

//         clip.set_volume(1.0);
//         clip.pause();
//         clip.detach();
//         std::thread::sleep(std::time::Duration::from_secs(60*5));

//         return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
//             audioname: i.to_string(),
//             isplay: true}
//         ).unwrap());
//     }

//     HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError{
//         error : "Song is already playing".to_string(),
//     }).unwrap())

// }

// #[post("/user/my/playlist/{search}/stop")]
// async fn stop_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let audio = get_audio();
//     for i in audio.keys(){

//         let file = BufReader::new(File::open(i).unwrap());
//         let clip = stream_handle.play_once(file).unwrap();

//         clip.set_volume(1.0);
//         clip.stop();
//         clip.detach();
//         std::thread::sleep(std::time::Duration::from_secs(60*3));
//         return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
//             audioname: i.to_string(),
//             isplay: true}
//         ).unwrap());
//     }

//     HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError{
//         error : "Song had stopped".to_string(),
//     }).unwrap())
// }

// #[post("/user/my/playlist/{search}/stepforward")]
// async fn stepforward_audio(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let audio = get_audio();
//     for i in audio.keys(){

//         let file = BufReader::new(File::open(i).unwrap());

//         let clip = stream_handle.play_once(file).unwrap();

//         clip.set_volume(1.0);
//         clip.skip_one();
//         clip.detach();
//         std::thread::sleep(std::time::Duration::from_secs(60*3));
//         return HttpResponse::Ok().body(hbr.render("music", &AudioSearchResults{
//             audioname: i.to_string(),
//             isplay: true}
//         ).unwrap());
//     }

//     HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError{
//         error : "No Song left".to_string(),
//     }).unwrap())
// }

#[post("/user/my/playlist/{search}/fastforward")]
async fn fastforward_audio(hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    
    // open audio file, with how many times audio will be played.
    // this function will increase the speed of audio file and then play it.
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = music::get_audio();
    for i in audio.keys() {
        let file = BufReader::new(File::open(i).unwrap());

        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.set_speed(2.0);
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60 * 3));

        return HttpResponse::Ok().body(
            hbr.render(
                "music",
                &AudioSearchResults {
                    audioname: i.to_string(),
                    isplay: true,
                },
            )
            .unwrap(),
        );
    }

    HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError {}).unwrap())
}

#[post("/user/my/playlist/{search}/fastbackward")]
async fn fastbackward_audio(hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    
    
    // open audio file, with how many times audio will be played.
    // this function will decrease the speed of audio file and then play it.
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = music::get_audio();
    for i in audio.keys() {
        let file = BufReader::new(File::open(i).unwrap());

        let clip = stream_handle.play_once(file).unwrap();

        clip.set_volume(1.0);
        clip.set_speed(0.7);
        clip.detach();
        std::thread::sleep(std::time::Duration::from_secs(60 * 3));

        return HttpResponse::Ok().body(
            hbr.render(
                "music",
                &AudioSearchResults {
                    audioname: i.to_string(),
                    isplay: true,
                },
            )
            .unwrap(),
        );
    }

    HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError {}).unwrap())
}

#[get("/user/library")]
async fn library() -> impl Responder {
    NamedFile::open_async("./static/collection.html").await
}

// You love good content with great quantity. You recently acknowledge that
// poor quantity not only create bad experience but also effect on your energy levels and ear working.
// And you asking this question over and over. We have a solution of your problem,
// you don't need media player connect with roombot and in collection section search song name .. wait for few milliseconds
// enjoy the song.  Another long feature each song categorize based on song emotion. The moment I knew [Taylor Swift] in love category.

#[post("/user/library/{searchbycollection}")]
async fn collection(
    form: web::Form<SearchPlaylist>,
    hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    
    
    // parse input values
    let query = &form.songname;

    // replace space with hypen
    // let q = &query.replace(" ", "-");
    let mut mp_player = query.to_owned() + ".mp3";

    // validate user session
    unsafe {
        
        let expire = gatekeeper::login_expire(ME);
        
        if expire {
            println!("Make sure you have provide correct information or session expired. ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    }

    

    // Here unsafe have a reason because of static calls.
    // The real problem is that retrive paticular song from library or collection, inorder to solve this problem
    // another problem we have faced called user might be skip format of song.

    // Query = [Lovely morning ] <=>  Record [Lovely-morning.mp3]

    unsafe {
        match mp_player.contains(".mp3") {
            true => {
                let mut record = music::new_beat(
                    mp_player.to_owned().to_string(),
                    Vec::<String>::new(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    false,
                    false,
                    false,
                    false,
                    false,
                    "".to_string(),
                    ME.to_string(),
                    0.0,
                );

                let client = match record.create_mongo_connection().await {
                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);

                let stream_record = record.get_song_from_playlist(db).await;

                let mut content = pinata_content::Content::new(
                    ME.to_string(),
                    "".to_string(),
                    "".to_string(),
                    stream_record.to_owned().song_name,
                    pinata_content::Emotionfilter::None,
                    false,
                    0,
                    0,
                );

                let client = match gatekeeper::mongodb_client().await {
                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);

                if content.session != stream_record.session {
                    let list = content.get_playlist_by_song(db.to_owned()).await;

                    let _data = GLOBAL_SONG.set(list.song.to_owned().to_string());
                    let _comment = MY_COMMENT.set(list.comment.to_owned().to_string());

                    

                    return HttpResponse::Ok().body(
                        hbr.render(
                            "search",
                            &SongEngine {
                                pmusic_artist: stream_record.artist[0].to_owned(),
                                pmusic_compose: stream_record.compose.to_owned(),
                                pmusic_genre: stream_record.genre.to_owned(),
                                pmusic_ilink: list.cid_icontent.to_owned(),
                                pmusic_lyric: stream_record.lyrics.to_owned(),
                                session: ME.to_string(),
                                name: stream_record.song_name.to_owned(),
                                pmusic_mlink: list.cid_mcontent.to_owned(),
                                pnumic_production: stream_record.studio_name.to_owned(),
                                favourite: list.like.to_owned(),
                                favourite_count: list.like_count.to_owned(),
                                played: list.play_count.to_owned(),
                                emotion: list.emotion.to_owned(),
                                comment : list.comment.to_owned(),
                                comment_like_count : list.comment_like_count.to_owned(),
                                comment_likes : list.comment_likes.to_owned(),
                                user_comments : list.followers_comments.to_owned(),
                                
                            },
                        )
                        .unwrap(),
                    );
                } else {
                    let list = content.get_playlist(db.to_owned()).await;

                    let _data = GLOBAL_SONG.set(list.song.to_owned().to_string());
                    let _comment = MY_COMMENT.set(list.comment.to_owned().to_string());

                    

                    return HttpResponse::Ok().body(
                        hbr.render(
                            "search",
                            &SongEngine {
                                pmusic_artist: stream_record.artist[0].to_owned(),
                                pmusic_compose: stream_record.compose.to_owned(),
                                pmusic_genre: stream_record.genre.to_owned(),
                                pmusic_ilink: list.cid_icontent.to_owned(),
                                pmusic_lyric: stream_record.lyrics.to_owned(),
                                session: ME.to_string(),
                                name: stream_record.song_name.to_owned(),
                                pmusic_mlink: list.cid_mcontent.to_owned(),
                                pnumic_production: stream_record.studio_name.to_owned(),
                                favourite: list.like.to_owned(),
                                favourite_count: list.like_count.to_owned(),
                                played: list.play_count.to_owned(),
                                emotion: list.emotion.to_owned(),
                                comment : list.comment.to_owned(),
                                comment_like_count : list.comment_like_count.to_owned(),
                                comment_likes : list.comment_likes.to_owned(),
                                user_comments : list.followers_comments.to_owned(),
                                
                            },
                        )
                        .unwrap(),
                    );
                }
            }
            false => {
                mp_player = mp_player.replace(".mp3", ".wav");
                let mut record = music::new_beat(
                    mp_player.to_owned().to_string(),
                    Vec::<String>::new(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    false,
                    false,
                    false,
                    false,
                    false,
                    "".to_string(),
                    ME.to_string(),
                    0.0,
                );

                let client = match record.create_mongo_connection().await {
                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);

                let stream_record = record.get_song_from_playlist(db).await;

                let mut content = pinata_content::Content::new(
                    ME.to_string(),
                    "".to_string(),
                    "".to_string(),
                    stream_record.to_owned().song_name,
                    pinata_content::genre_to_emotions(stream_record.genre.to_owned().to_string()),
                    false,
                    0,
                    0,
                );

                let client = match gatekeeper::mongodb_client().await {
                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);

                if content.session != stream_record.session {
                    let list = content.get_playlist(db.to_owned()).await;

                    let _data = GLOBAL_SONG.set(list.song.to_owned().to_string());
                    let _comment = MY_COMMENT.set(list.comment.to_owned().to_string());

                    

                    return HttpResponse::Ok().body(
                        hbr.render(
                            "search",
                            &SongEngine {
                                pmusic_artist: stream_record.artist[0].to_owned(),
                                pmusic_compose: stream_record.compose.to_owned(),
                                pmusic_genre: stream_record.genre.to_owned(),
                                pmusic_ilink: list.cid_icontent.to_owned(),
                                pmusic_lyric: stream_record.lyrics.to_owned(),
                                session: ME.to_string(),
                                name: stream_record.song_name.to_owned(),
                                pmusic_mlink: list.cid_mcontent.to_owned(),
                                pnumic_production: stream_record.studio_name.to_owned(),
                                favourite: list.like.to_owned(),
                                favourite_count: list.like_count.to_owned(),
                                played: list.play_count.to_owned(),
                                emotion: list.emotion.to_owned(),
                                comment : list.comment.to_owned(),
                                comment_like_count : list.comment_like_count.to_owned(),
                                comment_likes : list.comment_likes.to_owned(),
                                user_comments : list.followers_comments.to_owned(),
                                
                            },
                        )
                        .unwrap(),
                    );
                } else {
                    let list = content.get_playlist_by_song(db.to_owned()).await;

                    let _data = GLOBAL_SONG.set(list.song.to_owned().to_string());
                    let _comment = MY_COMMENT.set(list.comment.to_owned().to_string());

                    

                    return HttpResponse::Ok().body(
                        hbr.render(
                            "search",
                            &SongEngine {
                                pmusic_artist: stream_record.artist[0].to_owned(),
                                pmusic_compose: stream_record.compose.to_owned(),
                                pmusic_genre: stream_record.genre.to_owned(),
                                pmusic_ilink: list.cid_icontent.to_owned(),
                                pmusic_lyric: stream_record.lyrics.to_owned(),
                                session: ME.to_string(),
                                name: stream_record.song_name.to_owned(),
                                pmusic_mlink: list.cid_mcontent.to_owned(),
                                pnumic_production: stream_record.studio_name.to_owned(),
                                favourite: list.like.to_owned(),
                                favourite_count: list.like_count.to_owned(),
                                played: list.play_count.to_owned(),
                                emotion: list.emotion.to_owned(),
                                comment : list.comment.to_owned(),
                                comment_like_count : list.comment_like_count.to_owned(),
                                comment_likes : list.comment_likes.to_owned(),
                                user_comments : list.followers_comments.to_owned(),
                                
                            },
                        )
                        .unwrap(),
                    );
                }
            }
        }
    }
}

#[get("/user/composer")]
async fn artist() -> impl Responder {
    NamedFile::open_async("./static/artists.html").await
}

// You're an artist like Michael Jackson and you want to add your work? Will you try
// roombot ? Share your story with us.

#[post("/user/composer/newsong")]
async fn newsong_record(
    hbr: web::Data<Handlebars<'_>>,
    form: web::Form<MusicStream>,
) -> HttpResponse {
    

    // parse input values

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

    if lightnode_add == "" {
        println!("Make sure your account linked with light node address for secure transaction. ");
        return HttpResponse::BadRequest().body(hbr.render("error", &RequestError {}).unwrap());
    }

    // replace space with hypen
    // let q = &music_file.replace(" ", "-");

    // check whether session expire

   unsafe{
    
        let expire = gatekeeper::login_expire(ME);

        if expire {
        
            println!("Make sure you have provide correct information or session expired. ");
            return HttpResponse::BadRequest()
            .body(hbr.render("music_error", &RequestError {}).unwrap());
        }

    }

    

    let mut fees: f64 = 0.0;

    // read specific music file which is in music directory. If file is not in music diectory then throw error.
    // create music file record {music name, artists name, song type , production name etc}.
    // read cover image from download directory if exist;

    // store coverimage, music file on peer network and create content identifier address ;
    // which is then store back in database against session.
    if let Some(down_dir) = UserDirs::new() {
        if let Some(path) = down_dir.download_dir() {
            if !path.join(PathBuf::from(cover_img.to_owned())).exists() {
                println!("Make sure uploaded picture in Download Directory . ");
                return HttpResponse::BadRequest()
                    .body(hbr.render("music_error", &RequestError {}).unwrap());
            }

            let mut art: Vec<String> = Vec::<_>::new();
            art.push(artists.to_string());

            let mut earn: bool = false;
            let mut node: bool = false;
            let mut asset: bool = false;
            let mut fut: bool = false;
            let mut owner: bool = false;
            if royalty == "on"
                && lightnode == "on"
                && work == "on"
                && future == "on"
                && ownership == "on"
            {
                earn = true;
                node = true;
                asset = true;
                fut = true;
                owner = true;
            }

            unsafe {
                let mut record = music::new_beat(
                    music_file.to_owned().to_string(),
                    art.to_owned(),
                    cover_img.to_string(),
                    lightnode_add.to_string(),
                    date.to_string(),
                    lyrics.to_string(),
                    studio.to_string(),
                    genre.to_string(),
                    compose.to_string(),
                    website.to_string(),
                    endrosment.to_string(),
                    earn,
                    node,
                    asset,
                    fut,
                    owner,
                    email.to_string(),
                    ME.to_string(),
                    0.0,
                );

                let client = match record.create_mongo_connection().await {
                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);

                let _ = match record.create_collection(db).await {
                    Ok(collect) => collect,
                    Err(e) => panic!("{:?}", e),
                };

                let mut blob = ipinata::new_bolb_object(&path, ipinata::FileStatus::Pin);
                let pin_client = blob.pinta_client();

                let _auth = pin_client.test_authentication().await;
                
                let content = blob.upload_content(pin_client, cover_img.to_string()).await;

                let mut cid_image: String = "".to_string();
                if let Ok(object) = content {
                    cid_image = object.ipfs_hash;
                }

                println!("Image Content Indentifier {:?}", cid_image);

                let mut cid_music: String = "".to_string();

                if cid_image == "" {
                    return HttpResponse::BadRequest()
                        .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                let mpfile = ipinata::change_path(down_dir.to_owned(), music_file.to_owned());

                let mp_path = PathBuf::from(mpfile);

                let mut mp_blob = ipinata::new_bolb_object(&mp_path, ipinata::FileStatus::Pin);
                let connection = mp_blob.pinta_client();

                let mp_content = mp_blob
                    .upload_content(connection, music_file.to_owned().to_string())
                    .await;

                if let Ok(objects) = mp_content {
                    cid_music = objects.ipfs_hash;
                }

                println!("Music Content Indentifier {:?}", cid_music);

                let client = gatekeeper::mongodb_client().await;

                if let Ok(c) = client {
                    let mut content = pinata_content::Content::new(
                        ME.to_string(),
                        cid_image.to_owned().to_string(),
                        cid_music.to_owned().to_string(),
                        music_file.to_owned().to_string(),
                        pinata_content::genre_to_emotions(genre.to_owned().to_string()),
                        false,
                        0,
                        0,
                    );
                    let db = c.database(music::MUSIC_RECORD);

                    // once song added on ipfs , artist will pay contract finalize fees
                    if let Ok(_) = content.music_collection(db.to_owned()).await {
                        println!("Please wait content upload processing not take much time... ");

                        let mut nodeless = INodeless::new(
                            750,
                            email.to_owned().to_string(),
                            0.00,
                            art[0].to_owned().to_string(),
                            ME.to_owned().to_string(),
                            lightnode_net::TransactionStatus::Pending,
                            "".to_string(),
                        );
                        let node = nodeless.create_nodeless_client().await;
                        let status = node.to_owned().get_server_status().await;

                        println!(" Available = {:?}", status);

                        let store = nodeless.connect_with_store(&node.to_owned()).await;

                        if let Ok(digital_store) = store {
                            if digital_store.name.is_empty() {
                                println!("Make sure your account linked with light node address for secure transaction. ");
                                return HttpResponse::BadRequest()
                                    .body(hbr.render("error", &RequestError {}).unwrap());
                            }

                            let blockledge =
                                nodeless.lightnode_store_inovice(&node.to_owned()).await;

                            let db = c.database(music::MUSIC_RECORD);
                            let _ledger = nodeless.from_txs(db.to_owned()).await;

                            if let Ok(block) = blockledge {
                                let data = block.id.unwrap();
                                nodeless.lid = data.to_owned();
                                let _ = nodeless.update_tnx(db.to_owned()).await;

                                if let Ok(store_status) = nodeless.store_status(&node).await {
                                    println!("Inovice generate {:?}", store_status);

                                    let tx = nodeless.get_store_tnx(&node).await;

                                    fees = 750.00;

                                    if !tx.is_empty() {
                                        println!("Transaction status {:?}", tx[0].status);
                                    } else {
                                        println!("Make sure you have finalize your transaction");
                                        panic!("No Transaction return");
                                    }
                                }
                            }
                        }
                    } else {
                        return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                    }

                    return HttpResponse::Ok().body(
                        hbr.render(
                            "artists",
                            &Nftmint {
                                session: ME.to_string(),
                                song: music_file.to_owned().to_string(),
                                cid_image: cid_image.to_owned().to_string(),
                                cid_music: cid_music.to_owned().to_string(),
                                amount: fees.to_string(),
                            },
                        )
                        .unwrap(),
                    );
                }
            }
        }
    }

    HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError {}).unwrap())
}

#[post("/me/comment")]
async fn commenting(hbr : web::Data<Handlebars<'_>>, form: web::Form<Commenting>) -> HttpResponse{


    let comment = &form.icomment;
    
    
    
    if let Ok(client) = gatekeeper::mongodb_client().await{

        let db = client.database(music::MUSIC_RECORD);

        unsafe {

                if let Some(song) = GLOBAL_SONG.get(){

                    if song.to_owned().to_string().is_empty(){
                        println!("Make sure you don't submit empty form. ");
                        return HttpResponse::BadRequest()
                                    .body(hbr.render("music_error", &RequestError {}).unwrap());
                    }

                    let mut songdetails = pinata_content::Content::new(
                        ME.to_string(),
                        "".to_string(),
                        "".to_string(),
                        song.to_owned().to_string(),
                        pinata_content::Emotionfilter::None,
                        false,
                        0,
                        0,    
                    );

                    let content = songdetails.get_playlist_by_song(db.to_owned()).await;
                    
                    if comment.to_owned().to_string().is_empty(){
    
                        USERCOMMENTS += 0;
                        songdetails.comment = comment.to_owned().to_string();
                        
                        let _update = songdetails.update_song_info(db.to_owned()).await;
                        println!("Song details update....");
                    }else{
    
                        
                        USERCOMMENTS = content.followers_comments.to_owned()+1;
                        songdetails.comment = comment.to_owned().to_string();
                        songdetails.followers_comments = USERCOMMENTS;
                        
                        let _update = songdetails.update_song_info(db.to_owned()).await;
                        println!("Song details update....");
                    }
                }
            
        
        }
    }

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}


#[post("/me/comments/likes")]
async fn likes_on_comment(hbr : web::Data<Handlebars<'_>>) -> HttpResponse{

    if let Ok(client) = gatekeeper::mongodb_client().await{

        let db = client.database(music::MUSIC_RECORD);

        unsafe {

            if let Some(song) = GLOBAL_SONG.get(){

                if song.to_owned().to_string().is_empty(){
                    println!("Make sure you don't submit empty form. ");
                    return HttpResponse::BadRequest()
                                    .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                let mut songdetails = pinata_content::Content::new(
                    ME.to_string(),
                    "".to_string(),
                    "".to_string(),
                    song.to_owned().to_string(),
                    pinata_content::Emotionfilter::None,
                    false,
                    0,
                    0,    
                );

                let content = songdetails.get_playlist_by_song(db.to_owned()).await;

                if let Some(user_comment) = MY_COMMENT.get(){

                    
                    
                    if user_comment.to_owned().to_string().is_empty(){
                        
                        println!("Make sure user have comment before. ");
                        return HttpResponse::BadRequest()
                                    .body(hbr.render("music_error", &RequestError {}).unwrap());
                        
                    }else{
                        
                            
                            songdetails.comment_like_count += 1;
                            songdetails.comment_likes = true;
                            songdetails.comment = user_comment.to_owned().to_string();
                            songdetails.followers_comments = content.followers_comments+0;  
                            let _update = songdetails.update_song_info(db.to_owned()).await;
                            
                    }
                }
                   
            }
        }

    }

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}
// You will like or dislike song real time.

#[post("/me/like")]
async fn like_work(hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    
    let client = match gatekeeper::mongodb_client().await {
        Ok(list) => list,
        Err(e) => panic!("{:?}", e),
    };

    let db = client.database(music::MUSIC_RECORD);

    unsafe {
        
        if let Some(data) = GLOBAL_SONG.get() {
            
            if data.to_owned().to_string().is_empty() {
                
                println!("Make sure you don't submit empty form. ");
                return HttpResponse::BadRequest()
                    .body(hbr.render("music_error", &RequestError {}).unwrap());
            }

            let mut content = pinata_content::Content::new(
                ME.to_string(),
                "".to_string(),
                "".to_string(),
                data.to_owned().to_string(),
                pinata_content::Emotionfilter::None,
                false,
                0,
                0,
            );
            
            
            let old = content.get_playlist_by_song(db.to_owned()).await;
            LIKES = old.like_count;
            COLORED = old.like;
            PLAY = old.play_count;

            if LIKES >= 200 && PLAY >= 1000 {
                let mut nodeless = INodeless::new(
                    200,
                    "".to_owned().to_string(),
                    0.00,
                    "".to_owned().to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );
                let node = nodeless.create_nodeless_client().await;
                let status = node.to_owned().get_server_status().await;

                println!(" Available = {:?}", status);

                if let Ok(digital_store) = nodeless.connect_with_store(&node.to_owned()).await {
                    if digital_store.name.is_empty() {
                        println!("Make sure your account linked with light node address for secure transaction. ");
                        return HttpResponse::BadRequest()
                            .body(hbr.render("error", &RequestError {}).unwrap());
                    }

                    let db = client.database(music::MUSIC_RECORD);
                    let _ledger = nodeless.from_txs(db.to_owned()).await;

                    if let Ok(block) = nodeless.lightnode_store_inovice(&node.to_owned()).await {
                        let data = block.id.unwrap();
                        nodeless.lid = data.to_owned();
                        let _ = nodeless.update_tnx(db.to_owned()).await;

                        if let Ok(store_status) = nodeless.store_status(&node).await {
                            println!("Inovice generate {:?}", store_status);

                            let tx = nodeless.get_store_tnx(&node).await;

                            // fees = 750.00;

                            if !tx.is_empty() {
                                println!("Transaction status {:?}", tx[0].status);
                            } else {
                                println!(
                                    "Sorry Gateway have closed and you have to pay the charges"
                                );
                                return HttpResponse::BadRequest()
                                    .body(hbr.render("error", &RequestError {}).unwrap());
                            }
                        }
                    }
                }
            }

            if LIKES == 0 && !COLORED {
                
                LIKES += 1;
                COLORED = true;
                PLAY += 1;

                
                content.like_count = LIKES;
                content.play_count = PLAY;
                content.like = COLORED;

                let updater = content.update_song_info(db.to_owned()).await;

                print!("Content update {:?}", updater);
            } else {
                
                content.like_count = LIKES;
                content.play_count = PLAY;
                content.like = COLORED;

                let updater = content.update_song_info(db.to_owned()).await;

                print!("Content update {:?}", updater);
            }
        }
    }

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

#[get("/user/sociallink")]
async fn sociallink() -> impl Responder {
    NamedFile::open_async("./static/authlink.html").await
}

// Roombot provide easy way to connect with roombot , no need to remember 7bit long hex stream for the authenication.
#[post("/user/sociallink/profile")]
async fn profile(form: web::Form<Authenicate>, hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    // parse input values
    let username = &form.username;
    let email = &form.email;

    // transalate user input into secret code.
    // this code worked as account in our application
    // this code have many benefits ; code is used as authenication, authorization and validation

    //  that code sent back to database for future .

    let auth_code = gatekeeper::active_hash(&gatekeeper::new_profile(
        email.to_string(),
        username.to_string(),
    ));

    let mut auth = gatekeeper::Authenicate::new(auth_code.to_string(), username.to_string());

    unsafe {
        ME = auth_code;
    }

    let client = match gatekeeper::mongodb_client().await {
        Ok(list) => list,
        Err(e) => panic!("{:?}", e),
    };

    let db = client.database(music::MUSIC_RECORD);
    let _ = auth.create_record(db).await;

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

#[get("/user/poetry/topics")]
async fn add_topic() -> impl Responder {
    NamedFile::open_async("./static/poetry.html").await
}

#[post("/user/poetry/topics/{output}")]
async fn poetry(
    form: web::Form<TranslateFormData>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // parse input values
    let input: _ = &form.query;
    let apikey: _ = &form.call;

    // check whether any bad words exist in a query. then throw error
    let action = openai::validator(input.to_string());

    if let Ok(take_action) = action {
        if take_action {
            println!("Check your text there may be something which is not acceptable");

            HttpResponse::BadRequest().body(hbr.render("error", &RequestError {}).unwrap());
        }
    }

    // connect with openai call and complete the process

    if input.contains("poetry") {
        let mut opencall: _ = openai::new(
            input.to_string(),
            "".to_string(),
            input.len().try_into().unwrap(),
        );

        let responses = match opencall.openai_openend(apikey.to_string()).await {
            Ok(resp) => format!("{:?}", resp),
            Err(e) => panic!("Error = {:?}", e),
        };

        return HttpResponse::Ok().body(
            hbr.render(
                "translate",
                &ResponseTranslateForm {
                    query: input.to_string(),
                    response: responses,
                },
            )
            .unwrap(),
        );
    }

    println!("Check your text there may be something which is not acceptable");
    HttpResponse::BadRequest().body(hbr.render("error", &RequestError {}).unwrap())
}

#[get("/configurations")]
async fn configurations() -> impl Responder {
    NamedFile::open_async("./static/interactive.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // create handlebar new object, direct towards template directory. This direct used as reference for direction purpose.
    let mut handlebars_obj = Handlebars::new();
    handlebars_obj
        .register_templates_directory(".html", "./static/templates")
        .unwrap();

    // server hold handlebar template directory object value.
    let handlebars_ref = web::Data::new(handlebars_obj);

    // now server supported templates. These templates are render application state when a query execute.
    HttpServer::new(move || {
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
            .service(library)
            .service(collection)
            .service(like_work)
            .service(commenting)
            .service(likes_on_comment)
            // .service(stop_audio)
            // .service(stepforward_audio)
            .service(fastforward_audio)
            .service(fastbackward_audio)
            // .service(paused_audio)
            .service(artist)
            .service(newsong_record)
            .service(add_topic)
            .service(poetry)
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





