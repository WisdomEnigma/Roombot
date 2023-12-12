///
/// All the changes made according to wisdomenigma rules & MPL Licence terms.
///
/// Redistribution, Commitment of work, Licence of Work, Intellectual Property & trademark.   
///
///
/// Contact us
///   github.com/WisdomEnigma                   wizdwarfs@gmail.com

// development marco's 
#[warn(non_camel_case_types)]
#[warn(unused_imports)]
#[warn(unused_assignments)]


// import libraries
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use auth::gatekeeper;
use gpt_text::openai;
use serde::{Deserialize, Serialize};
// use img2vec::vec_middleware;
use core::panic;
use directories::UserDirs;
use handlebars::Handlebars;
use l2net::lightnode_net::{self, INodeless};
use movies::movies_rating::{Content, Emotionfilter, MovieRate};
use music_stream::{music, pinata_content};
use once_cell::sync::OnceCell;
use pinata_ipfs::{ipinata, ipfs_net};
use mongodb::Database;
use dotenv::dotenv;
use std::{path::PathBuf, env};

//  user submit their choices through html forms. 
// These forms have following implementation [Deserialize, Debug].

#[derive(Deserialize)]
struct TranslateFormData {
    query: String,
    call: String,
}

#[derive(Deserialize)]
struct SearchPlaylist {
    name: String,
}

#[derive(Deserialize)]
struct SearchMoviesPlaylist {
    name: String,
    year: String,
}

#[derive(Deserialize, Debug)]
struct EpisodeSearch{

    name : String,
}


#[derive(Deserialize)]
struct SearchArtist{

    name : String,
}

#[derive(Deserialize)]
struct SearchEmotion{

    name : String,
}



#[derive(Deserialize)]
struct Commenting {
    icomment: String,
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


#[derive(Deserialize)]
struct VirtualBook{

    name : String,
    isbn : String,
    publisher : String,
    pages : i64,
    description : String,
    author : String,
}

// User choices responses return html template for different html page's. 


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

#[derive(Serialize)]

struct MovieRecomend {
    title: String,
    genre_0: Emotionfilter,
    genre_1: Emotionfilter,
    genre_2: Emotionfilter,
    release: String,
    content: Content,
    watch_min: i64,
    official: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ITV{

    title : Vec::<String>,
    season : Vec::<u16>,
    episode : Vec::<u16>,
    imdb_id : Vec::<u32>,
    year :  Vec::<std::option::Option<u16>>,
    minutes : Vec::<std::option::Option<u16>>,

}


#[derive(Serialize, Debug)]
struct Playlist{

    song : Vec::<String>,
    cid_icontent : Vec::<String>,
    cid_mcontent : Vec::<String>,
    session : Vec::<String>,
    like : Vec::<bool>,
    like_count : Vec::<i64>,
    emotion : Vec::<pinata_content::Emotionfilter>,
    comment : Vec::<String>,
    comment_like_count : Vec::<i64>,
    comment_like : Vec::<bool>,
    followers_comments : Vec::<i64>,
    song_count : usize,
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
    comment: String,
    comment_like_count: i64,
    comment_likes: bool,
    user_comments: i64,
}

#[derive(Serialize)]
struct Recorded{

    name : String,
    episode : u16,
    seasons : u16,
    watch : u16,
    link : String, 
}




// static variables

static mut ME: u64 = 0;
static mut LIKES: i64 = 0;
static mut COLORED: bool = false;
static mut PLAY: i64 = 0;
static mut USERCOMMENTS: i64 = 0;
static GLOBAL_SONG: OnceCell<String> = OnceCell::new();
static MY_COMMENT: OnceCell<String> = OnceCell::new();
static ENV_TOKEN : OnceCell<String> = OnceCell::new();
static EMAIL : OnceCell<String> = OnceCell::new();
static SEARCHEPIC : OnceCell<String> = OnceCell::new();
static SEASONRELEASE : OnceCell<String> = OnceCell::new();




// routes

// 1. Index
#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}

// 2. Uptopia [Image]
#[get("/utopia")]
async fn image_utopia() -> impl Responder {
    NamedFile::open_async("./static/assets/utopia.jpg").await
}

// 3. Avatar [Image]
#[get("/user_avatar")]
async fn avatari() -> impl Responder {
    NamedFile::open_async("/home/ali/Downloads/register_face.png").await
}

// 4. Furturistic Learning [Image]
#[get("/futuristic")]
async fn image_learning() -> impl Responder {
    NamedFile::open_async("./static/assets/translation.png").await
}

// 5. Translation => get
#[get("/user/translation")]
async fn translator() -> impl Responder {
    NamedFile::open_async("./static/translate.html").await
}

// 5a. Translation => post
#[post("/user/translation/{output}")]
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

    let flag_words = responses.to_owned().len().eq(&10);
        
    if flag_words == true{

            let client = match gatekeeper::mongodb_client().await {
                Ok(list) => list,
                Err(e) => panic!("{:?}", e),
            };
    
            let db = client.database(music::MUSIC_RECORD);
            let fees : u64 = 25;     
            
            unsafe{
                let nodeless = INodeless::new(
                    fees,
                    "".to_owned().to_string(),
                    fees as f64,
                    "poetry composition".to_owned().to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );
    
                if let Ok(result) = payment_gateway(nodeless, db.to_owned()).await{
    
                    println!("Payment received {:?}", result);
                };
            }

            
        }

    HttpResponse::Ok().body(
        hbr.render(
            "translate",
            &ResponseTranslateForm {
                query: input.to_string(),
                response: responses.to_owned().to_string(),
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

// 6. Imovies => get
#[get("/user/imovies")]
async fn playlist() -> impl Responder {
    NamedFile::open_async("./static/movies.html").await
}

// 6a. Imovies => post
#[post("/user/recomend/imovies/{search}")]
async fn search_movies(
    form: web::Form<SearchMoviesPlaylist>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // parse input values
    let query = &form.name;
    let year = &form.year;

    let client = MovieRate::imdb_client().await;

    let yr = year.to_owned().to_string().parse::<u16>().unwrap();

    let mut genre: Vec<Emotionfilter> = Vec::<Emotionfilter>::new();

    genre.push(Emotionfilter::None);

    let mut imovies = MovieRate::new(
        query.to_owned().to_string(),
        yr,
        genre,
        "".to_owned().to_string(),
        Content::None,
        0,
    );

    let movies = imovies.imdb_movies(client).await;

    if let Some(imdb) = movies {
        imovies.imdb_id = imdb.imdb_id().to_owned().to_string();

        let _ = imovies.movies_iterator(imdb);

        if imovies.release as i64 <= 1975{

            unsafe{
               let mut record = music::new_beat(
                    "".to_owned().to_string(),
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


                let nodeless = INodeless::new(
                    100,
                    "".to_string(),
                    100.00,
                    "enjoy weekend with old stories".to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );

                let db = client.database(music::MUSIC_RECORD);
                if let Ok(result) = payment_gateway(nodeless, db.to_owned()).await{

                    println!("Payment received {:?}", result);
                };
            }
        }

        return HttpResponse::Ok().body(
            hbr.render(
                "movies",
                &MovieRecomend {
                    title: imovies.name.to_owned().to_string(),
                    genre_0: imovies.genre[0].to_owned(),
                    genre_1: imovies.genre[1].to_owned(),
                    genre_2: imovies.genre[2].to_owned(),
                    release: imovies.release.to_owned().to_string(),
                    content: imovies.adult.to_owned(),
                    watch_min: imovies.watch_min.to_owned() as i64,
                    official: imovies.official.to_owned().to_string(),
                },).unwrap(),);
    }
    

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

// web music player player your favourite song "The Moment I Knew". So phenomenal

// 7. Library => get

#[get("/user/library")]
async fn library() -> impl Responder {
    NamedFile::open_async("./static/collection.html").await
}

// You love good content with great quantity. You recently acknowledge that
// poor quantity not only create bad experience but also effect on your energy levels and ear working.
// And you asking this question over and over. We have a solution of your problem,
// you don't need media player connect with roombot and in collection section search song name .. wait for few milliseconds
// enjoy the song.  Another long feature each song categorize based on song emotion. The moment I knew [Taylor Swift] in love category.

// 7a. Library => post

#[post("/user/library/{searchbycollection}")]
async fn collection(
    form: web::Form<SearchPlaylist>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // parse input values
    let query = &form.name;

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

                if stream_record.song_name.to_owned().eq(&""){

                    return HttpResponse::BadRequest().body(hbr.render("error", &RequestError {}).unwrap());
                }

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
                                comment: list.comment.to_owned(),
                                comment_like_count: list.comment_like_count.to_owned(),
                                comment_likes: list.comment_likes.to_owned(),
                                user_comments: list.followers_comments.to_owned(),
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
                                comment: list.comment.to_owned(),
                                comment_like_count: list.comment_like_count.to_owned(),
                                comment_likes: list.comment_likes.to_owned(),
                                user_comments: list.followers_comments.to_owned(),
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
                

                if stream_record.song_name.to_owned().eq(&""){

                    return HttpResponse::BadRequest().body(hbr.render("error", &RequestError {}).unwrap());
                }

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
                                comment: list.comment.to_owned(),
                                comment_like_count: list.comment_like_count.to_owned(),
                                comment_likes: list.comment_likes.to_owned(),
                                user_comments: list.followers_comments.to_owned(),
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
                                comment: list.comment.to_owned(),
                                comment_like_count: list.comment_like_count.to_owned(),
                                comment_likes: list.comment_likes.to_owned(),
                                user_comments: list.followers_comments.to_owned(),
                            },
                        )
                        .unwrap(),
                    );
                }
            }
        }
    }
}

// 8 composer => get
#[get("/user/composer")]
async fn artist() -> impl Responder {
    NamedFile::open_async("./static/artists.html").await
}

// 8a. composer => post

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

    unsafe {
        let expire = gatekeeper::login_expire(ME);

        if expire {
            println!("Make sure you have provide correct information or session expired. ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    }

    
    let fees : i64 = 750;

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

                let mut blob = ipinata::new_blob_object(&path, ipinata::FileStatus::Pin);
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

                let mut mp_blob = ipinata::new_blob_object(&mp_path, ipinata::FileStatus::Pin);
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
                        
                        println!("Please wait content upload processing not take much time , when you pay fees ... ");

                        
                        let nodeless = INodeless::new(
                            fees as u64,
                            email.to_owned().to_string(),
                            fees as f64,
                            art[0].to_owned().to_string(),
                            ME.to_owned().to_string(),
                            lightnode_net::TransactionStatus::Pending,
                            "".to_string(),
                        );

                        if let Ok(result) = payment_gateway(nodeless, db.to_owned()).await{

                            println!("Payment received {:?}", result);
                        };
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
async fn commenting(hbr: web::Data<Handlebars<'_>>, form: web::Form<Commenting>) -> HttpResponse {
    let comment = &form.icomment;

    if let Ok(client) = gatekeeper::mongodb_client().await {
        let db = client.database(music::MUSIC_RECORD);

        unsafe {
            if let Some(song) = GLOBAL_SONG.get() {
                if song.to_owned().to_string().is_empty() {
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

                if comment.to_owned().to_string().is_empty() {
                    USERCOMMENTS += 0;
                    songdetails.comment = comment.to_owned().to_string();

                    let _update = songdetails.update_song_info(db.to_owned()).await;
                } else {
                    USERCOMMENTS = content.followers_comments.to_owned() + 1;
                    songdetails.comment = comment.to_owned().to_string();
                    songdetails.followers_comments = USERCOMMENTS;

                    let _update = songdetails.update_song_info(db.to_owned()).await;
                }
            }
        }
    }

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

// 9. comments_like => post
#[post("/me/comments/likes")]
async fn likes_on_comment(hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    if let Ok(client) = gatekeeper::mongodb_client().await {
        let db = client.database(music::MUSIC_RECORD);

        unsafe {
            if let Some(song) = GLOBAL_SONG.get() {
                if song.to_owned().to_string().is_empty() {
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

                if let Some(user_comment) = MY_COMMENT.get() {
                    if user_comment.to_owned().to_string().is_empty() {
                        println!("Make sure user have comment before. ");
                        return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                    } else {
                        songdetails.comment_like_count += 1;
                        songdetails.comment_likes = true;
                        songdetails.comment = user_comment.to_owned().to_string();
                        songdetails.followers_comments = content.followers_comments + 0;
                        let _update = songdetails.update_song_info(db.to_owned()).await;
                    }
                }
            }
        }
    }

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}
// You will like or dislike song real time.

// 10. like => post
#[post("/me/like")]
async fn like_work(hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    let client = match gatekeeper::mongodb_client().await {
        Ok(list) => list,
        Err(e) => panic!("{:?}", e),
    };

    let db = client.database(music::MUSIC_RECORD);
    let fees : u64 = 300;
    

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

            if LIKES >= 500 && PLAY >= 1000 {

                let nodeless = INodeless::new(
                    fees,
                    "".to_owned().to_string(),
                    fees as f64,
                    "1000 best songs seelctor".to_owned().to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );

                if let Ok(result) = payment_gateway(nodeless, db.to_owned()).await{

                    println!("Payment received {:?}", result);
                };            
                
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

// 11. sociallinks => get
#[get("/user/sociallink")]
async fn sociallink() -> impl Responder {
    NamedFile::open_async("./static/authlink.html").await
}

// 11a. sociallinks => post

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
        email.to_owned().to_string(),
        username.to_string(),
    ));


    let _ = EMAIL.set(email.to_owned());

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

// 12. poetry => get

#[get("/user/poetry/topics")]
async fn add_topic() -> impl Responder {
    NamedFile::open_async("./static/poetry.html").await
}

// 12a. poetry => post
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


        // check whether lines of generated response should be equal to 10 only then return true; if generated response words are less than 1000 then also return true;
        // in any case true then active payment gateway .

        let flag_lines = responses.to_owned().lines().count().eq(&10);
        let mut flag_words = responses.to_owned().len().le(&1000);
        
        if flag_lines == true || flag_words == true{

            let client = match gatekeeper::mongodb_client().await {
                Ok(list) => list,
                Err(e) => panic!("{:?}", e),
            };
    
            let db = client.database(music::MUSIC_RECORD);
            let fees : u64 = 100;     
            
            unsafe{
                let nodeless = INodeless::new(
                    fees,
                    "".to_owned().to_string(),
                    fees as f64,
                    "poetry composition".to_owned().to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );
    
                if let Ok(result) = payment_gateway(nodeless, db.to_owned()).await{
    
                    println!("Payment received {:?}", result);
                };
            }
   
        }

        // there may be possible iff generated response greater than 1000 then active payment gateway for transactiion.
        flag_words = responses.to_owned().len().ge(&1000);
        
        if flag_words == true{

            let client = match gatekeeper::mongodb_client().await {
                Ok(list) => list,
                Err(e) => panic!("{:?}", e),
            };
    
            let db = client.database(music::MUSIC_RECORD);
            let fees : u64 = 500;     
            
            unsafe{
                let nodeless = INodeless::new(
                    fees,
                    "".to_owned().to_string(),
                    fees as f64,
                    "poetry composition".to_owned().to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );
    
                if let Ok(result) = payment_gateway(nodeless, db.to_owned()).await{
    
                    println!("Payment received {:?}", result);
                };
            }

            
        }

        return HttpResponse::Ok().body(
            hbr.render(
                "translate",
                &ResponseTranslateForm {
                    query: input.to_owned().to_string(),
                    response: responses,
                },
            )
            .unwrap(),
        );
    }

    println!("Check your text there may be something which is not acceptable");
    HttpResponse::BadRequest().body(hbr.render("error", &RequestError {}).unwrap())
}

// 13. configuration => get

#[get("/configurations")]
async fn configurations() -> impl Responder {
    NamedFile::open_async("./static/interactive.html").await
}


// 14. ishows => get

#[get("/user/ishows")]
async fn shows() -> impl Responder {
    NamedFile::open_async("./static/episode.html").await
}

// 15. search_itv => post
#[post("/user/itvshows/{search}")]
async fn search_shows(
    form: web::Form<SearchMoviesPlaylist>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // parse input values
    let query = &form.name;
    let year = &form.year;


    // check whether user login through user credentials.
    unsafe {
        let expire = gatekeeper::login_expire(ME);

        if expire {
            println!("Make sure you have provide correct information or session expired. ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    }

    // generate movies client and wait till the process should not complete; 
    let client = MovieRate::imdb_client().await;

    // convert year into u16 format for further processing.
    let yr = year.to_owned().to_string().parse::<u16>().unwrap();

    // declaration of emotion filter which is pre-requisite. Initally no emotion processing , so none will be store.
    let mut genre: Vec<Emotionfilter> = Vec::<Emotionfilter>::new();

    genre.push(Emotionfilter::None);

    let mut imovies = MovieRate::new(
        query.to_owned().to_string(),
        yr,
        genre,
        "".to_owned().to_string(),
        Content::None,
        0,
    );


    let _ = SEARCHEPIC.set(query.to_owned().to_string());
    let _ = SEASONRELEASE.set(yr.to_owned().to_string());
    

    // look for relvant film/movies in the database; & active payment process 
    
    if let Some(itv) = imovies.imdb_season(client).await{
       
       let _ = imovies.tv_shows(itv).await;

       let client = match gatekeeper::mongodb_client().await {
            Ok(list) => list,
            Err(e) => panic!("{:?}", e),
        };

        let db = client.database(music::MUSIC_RECORD);
        let fees : u64 = 100;     

        unsafe{
            let nodeless = INodeless::new(
                fees,
                "".to_owned().to_string(),
                fees as f64,
                "user preference tv season".to_owned().to_string(),
                ME.to_owned().to_string(),
                lightnode_net::TransactionStatus::Pending,
                "".to_string(),
            );

            if let Ok(result) = payment_gateway(nodeless, db.to_owned()).await{

                println!("Payment received {:?}", result);
            };
        }
        
        return HttpResponse::Ok().body(hbr.render("tv", &MovieRecomend{
            
            
            title : imovies.name.to_owned(),
            genre_0 : imovies.genre[0].to_owned(),
            genre_1 : imovies.genre[1].to_owned(),
            genre_2 : imovies.genre[2].to_owned(),
            release : imovies.release.to_owned().to_string(),
            content : imovies.adult.to_owned(),
            watch_min : imovies.watch_min.to_owned() as i64,
            official : imovies.official.to_owned(),
        }).unwrap());
    }

    println!("Unfortunately Movie Title is not found");
    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}


// 16 search_epic => post

#[post("/user/itvshows/epic/{search}")]
async fn search_epic(form: web::Form<EpisodeSearch>,
    hbr: web::Data<Handlebars<'_>>) -> HttpResponse{

    let qsearch = &form.name;

    let client = MovieRate::imdb_client().await;

    let mut genre: Vec<Emotionfilter> = Vec::<Emotionfilter>::new();

    genre.push(Emotionfilter::None);

    if let Some(query) = SEARCHEPIC.get(){

        if let Some(year) = SEASONRELEASE.get(){

            let yr = year.to_owned().to_string().parse::<u16>().unwrap();

            let mut imovies = MovieRate::new(
                query.to_owned().to_string(),
                yr,
                genre,
                "".to_owned().to_string(),
                Content::None,
                0,
            );
        

        // look for hollywood seasons for database
            let bank =  imovies.get_episode(client).await;
            
            let (steps, flag) = imovies.get_episode_name(bank.to_owned(), qsearch.to_owned().to_string()).await;
            let show_label = imovies.get_episode_label(bank.to_owned(), steps, qsearch.to_owned().to_string()).await;
            let show_epic = imovies.get_episode_epic(bank.to_owned(), steps, qsearch.to_owned().to_string()).await;
            let show_watch = match imovies.get_episode_watch(bank.to_owned(), steps, qsearch.to_owned().to_string()).await{
                Some(min) => min,
                None => panic!("Error report"),
            };

            let show_id = imovies.get_episode_id(bank.to_owned(), steps, qsearch.to_owned().to_string()).await;

            // check whether all conditions meet 
            if (flag == true && show_label != 5000) && (show_epic != 5000 && show_watch != 5000) && (show_id != 5000){

                return HttpResponse::Ok().body(hbr.render("epic", &Recorded {
                    name : qsearch.to_owned().to_string(),
                    episode : show_label,
                    seasons : show_epic,
                    watch : show_watch,
                    link :  ("https://www.imdb.com/title/tt".to_string() + &show_id.to_owned().to_string() + &"/mediaindex/?ref_=tt_mv_sm".to_string()),
                }).unwrap());            
                
            }
        }

        
    }
    
    println!("Unfortunately Show Title is not found");
    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}


// 17. search_artist => post
#[post("/user/library/{search}/{artist}")]
async fn search_artist(form: web::Form<SearchArtist>,
    hbr: web::Data<Handlebars<'_>>) -> HttpResponse{

        let asearch = &form.name;

        let mut art = Vec::<String>::new();
        art.push(asearch.to_owned().to_string());

        unsafe{

                // validate user session
                let expire = gatekeeper::login_expire(ME);

                if expire {
                    
                    println!("Make sure you have provide correct information or session expired. ");
                    return HttpResponse::BadRequest()
                                .body(hbr.render("music_error", 
                                            &RequestError {}).unwrap());
                }

                let mut record = music::new_beat(
                    "".to_owned().to_string(),
                    art,
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
                    0.0);

                let client = match record.create_mongo_connection().await {
                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);

                let stream_record = record.get_song_from_playlist_through_artist(db).await;

                // check whether artist name present in our database, if not then retry again 

                if stream_record.len() > 0 && stream_record[stream_record.len()-1].song_name.to_owned().eq(&""){

                    println!("Artist yet not made content on our platform , hopefully next time .. ");
                    return HttpResponse::Ok()
                                    .body(hbr.render("collection", 
                                            &Homepage {}).unwrap());
                }

                let mut iterate = stream_record.to_owned().into_iter();

                let mut record = Vec::<music::MusicRecord>::new();

                for music in iterate.by_ref(){

                    if music.song_name.to_owned().eq(&""){
                        continue; 
                    }

                    record.push(music);
                }
            
                let mut content = pinata_content::Content::new(
                    ME.to_string(),
                    "".to_string(),
                    "".to_string(),
                    record[record.len()-1].song_name.to_owned().to_string(),
                    pinata_content::genre_to_emotions(stream_record[record.len()-1].genre.to_owned().to_string()),
                    false,
                    0,
                    0,
                );


                let client = match gatekeeper::mongodb_client().await {
                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);


                

                let playlist_song = content.get_playlist_by_song(db).await;
                
                return HttpResponse::Ok().body(
                    hbr.render("search",
                                &SongEngine {
                                        pmusic_artist: record[record.len()-1].artist[0].to_owned(),
                                        pmusic_compose: record[record.len()-1].compose.to_owned(),
                                        pmusic_genre:   record[record.len()-1].genre.to_owned(),
                                        pmusic_ilink: playlist_song.cid_icontent.to_owned(),
                                        pmusic_lyric: record[record.len()-1].lyrics.to_owned(),
                                        session: ME.to_string(),
                                        name: record[record.len()-1].song_name.to_owned(),
                                        pmusic_mlink: playlist_song.cid_mcontent.to_owned(),
                                        pnumic_production: record[record.len()-1].studio_name.to_owned(),
                                        favourite: playlist_song.like.to_owned(),
                                        favourite_count: playlist_song.like_count.to_owned(),
                                        played: playlist_song.play_count.to_owned(),
                                        emotion: playlist_song.emotion.to_owned(),
                                        comment: playlist_song.comment.to_owned(),
                                        comment_like_count: playlist_song.comment_like_count.to_owned(),
                                        comment_likes: playlist_song.comment_likes.to_owned(),
                                        user_comments: playlist_song.followers_comments.to_owned(),
                        
                }).unwrap());
            }
        
        }
        

        // 19. search_emotion => post
#[post("/user/library/{search}/{music}/{emotion}")]
async fn search_emotion(form: web::Form<SearchEmotion>,
    hbr: web::Data<Handlebars<'_>>) -> HttpResponse{

        
        let emo = &form.name;


        // warning : search emotion functionality allow you to listen base on your emotion. There maybe possible user are depressed then no song will be played;

        if emo.to_owned().eq(&"Depressed") || emo.to_owned().eq(&"Sucide") {

            println!("Please visit doctor , medicate & listen light hearted songs");
            return HttpResponse::Ok()
                                    .body(hbr.render("collection", 
                                            &Homepage {}).unwrap());
        }

        unsafe{

            // validate user session
            let expire = gatekeeper::login_expire(ME);

            if expire {
                
                println!("Make sure you have provide correct information or session expired. ");
                return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", 
                                        &RequestError {}).unwrap());
            }

            let mut content = pinata_content::Content::new(
                ME.to_string(),
                "".to_string(),
                "".to_string(),
                "".to_owned().to_string(),
                pinata_content::genre_to_emotions(emo.to_owned().to_string()),
                false,
                0,
                0,
            );

            let client = match gatekeeper::mongodb_client().await {
                Ok(list) => list,
                Err(e) => panic!("{:?}", e),
            };

            let db = client.database(music::MUSIC_RECORD);

            let records = content.get_playlist_through_beat(db, emo.to_owned().to_string()).await;

            if records.len() == 0 {

                    println!(" This emotion has no such playlist , hopefully next time .. ");
                    return HttpResponse::Ok()
                                    .body(hbr.render("collection", 
                                            &Homepage {}).unwrap());
                }

            let mut list = Playlist{
                song : Vec::<String>::new(), 
                session : Vec::<String>::new(), 
                cid_icontent : Vec::<String>::new(), 
                like : Vec::<bool>::new(), 
                cid_mcontent : Vec::<String>::new(), 
                like_count : Vec::<i64>::new(), 
                emotion : Vec::<pinata_content::Emotionfilter>::new(), 
                comment: Vec::<String>::new(), 
                comment_like : Vec::<_>::new(), 
                comment_like_count : Vec::<i64>::new(), 
                followers_comments : Vec::<i64>::new(),
                song_count: 0,
            };


            
            let mut it = records.to_owned().into_iter();


            for data in it.by_ref(){

                let song = data.song.clone();
                let session = data.session.clone();
                let cid_icontent = data.cid_icontent.clone();
                let cid_mcontent = data.cid_mcontent.clone();
                let like = data.like.clone();
                let like_count = data.like_count.clone();
                let comment_likes = data.comment_likes.clone();
                let comment = data.comment.clone();
                let emotion = data.emotion.clone();
                let comment_like_count = data.comment_like_count.clone();
                let followers_comments = data.followers_comments.clone();

                list.song_count = records.len();

                list.song.push(song);
                list.session.push(session);
                list.cid_icontent.push(cid_icontent);
                list.cid_mcontent.push(cid_mcontent);
                list.comment.push(comment);
                list.like.push(like);
                list.comment_like.push(comment_likes);
                list.like_count.push(like_count);
                list.emotion.push(emotion);
                list.comment_like_count.push(comment_like_count);
                list.followers_comments.push(followers_comments);
            }


            let client = match gatekeeper::mongodb_client().await {
                Ok(list) => list,
                Err(e) => panic!("{:?}", e),
            };
    
            let db = client.database(music::MUSIC_RECORD);
            let fees : u64 = 300;

            let nodeless = INodeless::new(
                fees,
                "".to_owned().to_string(),
                fees as f64,
                "search your emotion in our playlist... it's too hard ".to_owned().to_string(),
                ME.to_owned().to_string(),
                lightnode_net::TransactionStatus::Pending,
                "".to_string(),
            );

            if let Ok(result) = payment_gateway(nodeless, db.to_owned()).await{

                println!("Payment received {:?}", result);
            };

            return HttpResponse::Ok().body(hbr.render("emotions", &list).unwrap());
        }
        
}


// 20. virtual book => get

#[get("/user/library/books")]
async fn virtual_book() -> impl Responder {
    
    NamedFile::open_async("./static/add_books.html").await
}


// 21. add book
#[post("/user/library/books/{add}")]
async fn add_virtual_book(form: web::Form<VirtualBook>,
    hbr: web::Data<Handlebars<'_>>) -> HttpResponse{

        let title = &form.name;
        let author = &form.author;
        let pages = &form.pages;
        let description = &form.description;
        let isbn = &form.isbn;
        let publisher = &form.publisher;

        let mut instance =  ipfs_net::IpfsBucket::new(title.to_owned().to_string());
        let path = instance.get_file_path();

        println!("File Path {:?}", path);

        println!("Author {:?}, Pages {:?}, Description {:?}, Isbn {:?}, Publisher {:?}", author, pages, description, isbn, publisher);


        return HttpResponse::Ok().body(hbr.render("home", &Homepage{}).unwrap());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // These lines allow to read secrets that is require to complete the process.
    dotenv().ok();    
    let _token = ENV_TOKEN.set(env::var("OPENAI_API_KEY").expect("token is not provided").to_string());
    
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
            .service(search_movies)
            .service(library)
            .service(collection)
            .service(like_work)
            .service(commenting)
            .service(likes_on_comment)
            .service(artist)
            .service(newsong_record)
            .service(add_topic)
            .service(poetry)
            .service(configurations)
            .service(sociallink)
            .service(profile)
            .service(shows)
            .service(search_shows)
            .service(search_epic)
            .service(search_artist)
            .service(search_emotion)
            .service(virtual_book)
            .service(add_virtual_book)
        // .service(register_user)
        // .service(register_face)
        // .service(login)
        // .service(login_account)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


/// this function allow to user to complete the transaction process with in few seconds.
/// First application connected with internet then generate inovice which is available for few seconds.  
/// User will deposit requested satoshi's and automatically proceed the process.

pub async fn payment_gateway(mut nodeless : INodeless, db : Database) -> std::io::Result<()> {

    let accept : bool = false;
    
    let node = nodeless.create_nodeless_client().await;
    let status = node.to_owned().get_server_status().await;

    if let Ok(digital_store) = nodeless.connect_with_store(&node.to_owned()).await {
        
        if digital_store.name.is_empty() {
            
            println!("Make sure you connect with internet {:?} ", status);
            panic!("Make sure you connect with internet");
        }

        
        let _ledger = nodeless.from_txs(db.to_owned()).await;

        if let Ok(block) = nodeless.lightnode_store_inovice(&node.to_owned()).await {
            
            let data = block.id.unwrap();
            nodeless.lid = data.to_owned();

            if let Some(email) = EMAIL.get(){

                nodeless.email = email.to_owned().to_string();
            }
            
            
            let _ = nodeless.update_tnx(db.to_owned()).await;

            if let Ok(store_status) = nodeless.store_status(&node).await {
                
                println!("Inovice generate {:?}", store_status);

                let tx = nodeless.get_store_tnx(&node).await;                

                if !tx.is_empty() && !accept {
                    
                    println!("Transaction status {:?}", tx[0].status);
                    
                    nodeless.status = lightnode_net::TransactionStatus::Deposit;
                    nodeless.remaining = 0.00;
                    
                    let _ = nodeless.update_tnx(db.to_owned()).await;
                    
                    return Ok(());
                
                } else {
                    
                    nodeless.status = lightnode_net::TransactionStatus::Expire;
                    nodeless.remaining = nodeless.amount as f64;

                    let _ = nodeless.update_tnx(db.to_owned()).await;

                    println!("Sorry Gateway has closed and kindly retry this operation {:?}", tx[0]);
                    println!("visit https://nodeless.io/app/stores/dashboard/e1be7458-9364-4f40-8de0-22a3d5af8db5/ for further information");
                    panic!("Payment gateway has closed retry this operation {:?}", nodeless.status);
                }
            }
        }
    }

    Ok(())
}
