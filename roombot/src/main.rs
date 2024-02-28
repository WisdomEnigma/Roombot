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
use dotenv::dotenv;
use handlebars::Handlebars;
use l2net::lightnode_net::{self, INodeless};
use mongodb::Database;
use movies::movies_rating::{Content, Emotionfilter, MovieRate};
use music_stream::{music, pinata_content};
use once_cell::sync::OnceCell;
use pinata_ipfs::{ipfs_net, ipinata};
use std::{env, path::PathBuf};

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
struct EpisodeSearch {
    name: String,
}

#[derive(Deserialize)]
struct SearchArtist {
    name: String,
}

#[derive(Deserialize)]
struct SearchEmotion {
    name: String,
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
struct EditAccount {
    name: String,
    lastname: String,
    sname: String, // school name
    degree: String,
    cname: String, // campany name
    work: String,
    city: String,
    country: String,
    bitcoin: String,

    address : String,
    fblink : String,
    instalink : String,
    xlink : String,
    youlink : String,
    new_digitalverse : String,
    old_digitalverse : String,
    phone : String,
}

#[derive(Deserialize, Debug)]

struct SearchParam {
    query: String,
}

#[derive(Deserialize)]
struct VirtualBook {
    name: String,
    isbn: String,
    publisher: String,
    pages: i64,
    description: String,
    author: String,
}

#[derive(Deserialize)]
struct Booksearch {
    bookname: String,
}

#[derive(Deserialize)]
struct Discover{

    discover : String,
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
struct ITV {
    title: Vec<String>,
    season: Vec<u16>,
    episode: Vec<u16>,
    imdb_id: Vec<u32>,
    year: Vec<std::option::Option<u16>>,
    minutes: Vec<std::option::Option<u16>>,
}

#[derive(Serialize, Debug)]
struct Playlist {
    song: Vec<String>,
    cid_icontent: Vec<String>,
    cid_mcontent: Vec<String>,
    session: Vec<String>,
    like: Vec<bool>,
    like_count: Vec<i64>,
    emotion: Vec<pinata_content::Emotionfilter>,
    comment: Vec<String>,
    comment_like_count: Vec<i64>,
    comment_like: Vec<bool>,
    followers_comments: Vec<i64>,
    song_count: usize,
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
struct Recorded {
    name: String,
    episode: u16,
    seasons: u16,
    watch: u16,
    link: String,
}

#[derive(Serialize, Debug)]
struct Searched {
    name: Vec<String>,
    counter: String,
    leads: String,
    follower: String,
    session: String,
}

#[derive(Serialize, Debug)]

struct GetBook {
    name: String,
    page: String,
    description: String,
    author: String,
    publisher: String,
    session: String,
    ipfs_link: String,
}

#[derive(Serialize, Debug)]

struct DiscoverPersonality{

    firstname : String,
    lastname : String,
    address : Vec::<String>,
    city : String,
    country : String,
    bitcoinwallet : Vec::<String>,
    workplace : Vec::<String>,
    degree : Vec::<String>,
    insitute : Vec::<String>,
    fblink : String,
    instalink : String,
    youlink : String,
    xlink : String,
    avatar : Vec::<String>, 
    career : String,
    session : String,
    new_digital : Vec::<String>,
    phone : String,
}

// static variables

static mut ME: u64 = 0;
static mut LIKES: i64 = 0;
static mut COLORED: bool = false;
static mut PLAY: i64 = 0;
static mut USERCOMMENTS: i64 = 0;
static GLOBAL_SONG: OnceCell<String> = OnceCell::new();
static MY_COMMENT: OnceCell<String> = OnceCell::new();
static ENV_TOKEN: OnceCell<String> = OnceCell::new();
static EMAIL: OnceCell<String> = OnceCell::new();
static SEARCHEPIC: OnceCell<String> = OnceCell::new();
static SEASONRELEASE: OnceCell<String> = OnceCell::new();
static MY_BITCOIN_ADDR: OnceCell<String> = OnceCell::new();

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

    if flag_words == true {
        let client = match gatekeeper::mongodb_client().await {
            Ok(list) => list,
            Err(e) => panic!("{:?}", e),
        };

        let db = client.database(music::MUSIC_RECORD);
        let fees: u64 = 25;

        unsafe {
            let nodeless = INodeless::new(
                fees,
                "".to_owned().to_string(),
                fees as f64,
                "translate language".to_owned().to_string(),
                ME.to_owned().to_string(),
                lightnode_net::TransactionStatus::Pending,
                "".to_string(),
            );

            let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
            if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string()){

                println!("Nodeless Bitcoin Gateway down");
                return HttpResponse::BadRequest()
                        .body(hbr.render("music_error", &RequestError {}).unwrap());
            }

            if status
                .to_owned()
                .to_string()
                .eq(&"Device is not connected with internet ".to_string())
            {
                println!("Internet disconnect ");
                return HttpResponse::BadRequest()
                    .body(hbr.render("music_error", &RequestError {}).unwrap());
            }

            if status.to_owned().to_string().eq(&"Payment acccept") {
                println!("Payment Accepted ");
                println!(
                    "Result ready {:?} ",
                    status.to_owned().to_string().eq(&"Payment acccept")
                );
            }
        }

        let _gateway = match direct_gateway(fees).await {
            Ok(_) => {
                return HttpResponse::Ok().body(
                    hbr.render(
                        "translate",
                        &ResponseTranslateForm {
                            query: input.to_string(),
                            response: responses.to_owned().to_string(),
                        },
                    )
                    .unwrap(),
                );
            }

            Err(e) => {
                eprintln!("Error {:?}", e);

                return HttpResponse::BadRequest()
                    .body(hbr.render("music_error", &RequestError {}).unwrap());
            }
        };
    }

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
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

    // create movies database client which allow to access movies information

    let movies = imovies.imdb_movies(client).await;

    if let Some(imdb) = movies {
        imovies.imdb_id = imdb.imdb_id().to_owned().to_string();

        let fees = 100;

        let _ = imovies.movies_iterator(imdb);

        // check whether film release less than 1975; then open payment gateway and collect charges before provide information

        if imovies.release as i64 <= 1975 {
            unsafe {
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
                    fees,
                    "".to_string(),
                    fees as f64,
                    "enjoy weekend with old stories".to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );

                let db = client.database(music::MUSIC_RECORD);
                let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
                if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string()){

                        println!("Nodeless Bitcoin Gateway down");
                        return HttpResponse::BadRequest()
                                .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                if status
                    .to_owned()
                    .to_string()
                    .eq(&"Device is not connected with internet ".to_string())
                {
                    println!("Internet disconnect ");
                    return HttpResponse::BadRequest()
                        .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                if status.to_owned().to_string().eq(&"Payment acccept") {
                    println!("Payment Accepted ");
                    println!(
                        "Result ready! {:?} ",
                        status.to_owned().to_string().eq(&"Payment acccept")
                    );

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
                            },
                        )
                        .unwrap(),
                    );
                }

                let _gateway = match direct_gateway(fees).await {
                    Ok(_) => {
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
                                },
                            )
                            .unwrap(),
                        );
                    }

                    Err(e) => {
                        eprintln!("Error {:?}", e);
                        return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                    }
                };
            }
        }
    }

    // if condition false then render homepage.
    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

// 7. Library => get

#[get("/user/library")]
async fn library() -> impl Responder {
    NamedFile::open_async("./static/collection.html").await
}

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
        // there may be possible that song store in mp3 format or anyother format.
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

                // search song , whether it will be exist or throw error if not.

                let client = match record.create_mongo_connection().await {
                    Ok(list) => list,
                    Err(e) => panic!("{:?}", e),
                };

                let db = client.database(music::MUSIC_RECORD);

                let stream_record = record.get_song_from_playlist(db).await;

                if stream_record.song_name.to_owned().eq(&"") {
                    return HttpResponse::BadRequest()
                        .body(hbr.render("error", &RequestError {}).unwrap());
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

                // now check whether artist engage with his content or user ; if user then return song details with file format for user.
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

                if stream_record.song_name.to_owned().eq(&"") {
                    return HttpResponse::BadRequest()
                        .body(hbr.render("error", &RequestError {}).unwrap());
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

    if lightnode_add.to_owned().eq(&"") {
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

    let fees: i64 = 750;

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

            // upload music & song cover image on pinata ipfs. Music must be exist in Music while cover image should be in downloads
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

                // ping ...
                let _auth = pin_client.test_authentication().await;

                // cover image upload
                let content = blob.upload_content(pin_client, cover_img.to_string()).await;

                // get content routing address of cover image
                let mut cid_image: String = "".to_string();
                if let Ok(object) = content {
                    cid_image = object.ipfs_hash;
                }

                println!("Image Content Indentifier {:?}", cid_image);

                // now do it with music
                let mut cid_music: String = "".to_string();

                if cid_image.to_owned().eq(&"") {
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

                // active payment gateway and collect charges
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

                        let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
                        if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string()){

                            println!("Nodeless Bitcoin Gateway down");
                            return HttpResponse::BadRequest()
                                    .body(hbr.render("music_error", &RequestError {}).unwrap());
                        }

                        if status
                            .to_owned()
                            .to_string()
                            .eq(&"Device is not connected with internet ".to_string())
                        {
                            println!("Internet disconnect ");
                            return HttpResponse::BadRequest()
                                .body(hbr.render("music_error", &RequestError {}).unwrap());
                        }

                        if status.to_owned().to_string().eq(&"Payment acccept") {
                            println!("Payment Accepted ");
                            println!(
                                "Result ready! {:?} ",
                                status.to_owned().to_string().eq(&"Payment acccept")
                            );
                        }

                        let _ = match direct_gateway(fees as u64).await {
                            Ok(_) => {
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

                            Err(e) => {
                                eprintln!("Error {:?}", e);
                                return HttpResponse::BadRequest()
                                    .body(hbr.render("music_error", &RequestError {}).unwrap());
                            }
                        };
                    } else {
                        return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                    }
                }
            }
        }
    }

    // there may be possible that application cash for invalid choice ..
    HttpResponse::BadRequest().body(hbr.render("music_error", &RequestError {}).unwrap())
}

#[post("/me/comment")]
async fn commenting(hbr: web::Data<Handlebars<'_>>, form: web::Form<Commenting>) -> HttpResponse {
    // user comment
    let comment = &form.icomment;

    if let Ok(client) = gatekeeper::mongodb_client().await {
        let db = client.database(music::MUSIC_RECORD);

        unsafe {
            // check whether song already exist in a record ; if so then

            if let Some(song) = GLOBAL_SONG.get() {
                // does song remain exist in a record
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

                // get song from record

                let content = songdetails.get_playlist_by_song(db.to_owned()).await;

                // check whether comment should be empty ; then remain unchanged record; otherwise
                if comment.to_owned().to_string().is_empty() {
                    USERCOMMENTS += 0;
                    songdetails.comment = comment.to_owned().to_string();

                    let _update = songdetails.update_song_info(db.to_owned()).await;
                } else {
                    // update the record, by adding comment
                    USERCOMMENTS = content.followers_comments.to_owned() + 1;
                    songdetails.comment = comment.to_owned().to_string();
                    songdetails.followers_comments = USERCOMMENTS;

                    let _update = songdetails.update_song_info(db.to_owned()).await;
                }
            }
        }
    }

    // there may be possible application may be crash for any invalid or poor choice
    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

// 9. comments_like => post
#[post("/me/comments/likes")]
async fn likes_on_comment(hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    if let Ok(client) = gatekeeper::mongodb_client().await {
        let db = client.database(music::MUSIC_RECORD);

        unsafe {
            // get song from record & also user like to listen & engage trough comments & like on it.
            if let Some(song) = GLOBAL_SONG.get() {
                // check whether song should be empty then throw error
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
                    // check whether commment should not empty
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

    // there maybe disconnect internet during session
    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

// 10. like => post
#[post("/me/like")]
async fn like_work(hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    // check whether app have cloud database access
    let client = match gatekeeper::mongodb_client().await {
        Ok(list) => list,
        Err(e) => panic!("{:?}", e),
    };

    let db = client.database(music::MUSIC_RECORD);
    let fees: u64 = 300;

    unsafe {
        // get the song from cloud database...
        if let Some(data) = GLOBAL_SONG.get() {
            // check whether data should not empty
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

            // check whether Likes greater equal 500 && Playing song greater than 1000, active payment gateway and collect service charges.
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

                let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
                if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string()){

                        println!("Nodeless Bitcoin Gateway down");
                        return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                if status
                    .to_owned()
                    .to_string()
                    .eq(&"Device is not connected with internet ".to_string())
                {
                    println!("Internet disconnect ");
                    return HttpResponse::BadRequest()
                        .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                if status.to_owned().to_string().eq(&"Payment acccept") {
                    println!("Payment Accepted ");
                    println!(
                        "Result ready {:?} ",
                        status.to_owned().to_string().eq(&"Payment acccept")
                    );

                    return HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap());
                }

                let _gateway = match direct_gateway(fees).await {
                    Ok(_) => {
                        return HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap());
                    }

                    Err(e) => {
                        eprintln!("Error {:?}", e);
                        return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                    }
                };
            }

            // check whether any user like the song who will not rate song before, then update
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
                // no record should be update..
                content.like_count = LIKES;
                content.play_count = PLAY;
                content.like = COLORED;

                let updater = content.update_song_info(db.to_owned()).await;

                print!("Content update {:?}", updater);
            }
        }
    }

    // there may be possible that app do not have database due to internet lost
    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

// 11. sociallinks => get
#[get("/user/sociallink")]
async fn sociallink() -> impl Responder {
    NamedFile::open_async("./static/authlink.html").await
}

// 11a. sociallinks => post

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

    // check whether user have profile or new user.
    if let Ok(client) = gatekeeper::mongodb_client().await {
        let db = client.database(music::MUSIC_RECORD);
        let _ = auth.create_record(db).await;

        let mut user = auth::accounts::Info::new(
            "".to_owned().to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        );

        let minit = user.mongo_init().await;
        let access = user.access_credentials(minit);

        unsafe {
            user.set_session(ME.to_owned().to_string());
        }

        let tx_status = user.transaction_status(access.to_owned()).await.unwrap();

        if tx_status.to_owned().to_string().eq(&"No record") {
            eprintln!("Error [No User]");
        } else if tx_status
            .to_owned()
            .to_string()
            .eq(&"No bitcoin address provided")
        {
            eprintln!("Error [You don't have secure wallet address, Ouuch! You're missing MARVELOUS Experience]");
        } else {
            let _ = MY_BITCOIN_ADDR.set(tx_status.to_owned());
        }
    };

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

        if flag_lines == true || flag_words == true {
            let client = match gatekeeper::mongodb_client().await {
                Ok(list) => list,
                Err(e) => panic!("{:?}", e),
            };

            let db = client.database(music::MUSIC_RECORD);
            let fees: u64 = 100;

            unsafe {
                let nodeless = INodeless::new(
                    fees,
                    "".to_owned().to_string(),
                    fees as f64,
                    "poetry composition".to_owned().to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );

                let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
                if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string()){

                    println!("Nodeless Bitcoin Gateway down");
                    return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                if status
                    .to_owned()
                    .to_string()
                    .eq(&"Device is not connected with internet ".to_string())
                {
                    println!("Internet disconnect ");
                    return HttpResponse::BadRequest()
                        .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                if status.to_owned().to_string().eq(&"Payment acccept") {
                    println!("Payment Accepted ");
                    println!(
                        "Result ready! {:?} ",
                        status.to_owned().to_string().eq(&"Payment acccept")
                    );

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

                let _ = match direct_gateway(fees).await {
                    Ok(_) => {
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
                    Err(e) => {
                        eprintln!("Error {:?}", e);
                        return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                    }
                };
            }
        }

        // there may be possible iff generated response greater than 1000 then active payment gateway for transactiion.
        flag_words = responses.to_owned().len().ge(&1000);

        if flag_words == true {
            let client = match gatekeeper::mongodb_client().await {
                Ok(list) => list,
                Err(e) => panic!("{:?}", e),
            };

            let db = client.database(music::MUSIC_RECORD);
            let fees: u64 = 500;

            unsafe {
                let nodeless = INodeless::new(
                    fees,
                    "".to_owned().to_string(),
                    fees as f64,
                    "poetry composition".to_owned().to_string(),
                    ME.to_owned().to_string(),
                    lightnode_net::TransactionStatus::Pending,
                    "".to_string(),
                );

                let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
                if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string()){

                        println!("Nodeless Bitcoin Gateway down");
                        return HttpResponse::BadRequest()
                                .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                if status
                    .to_owned()
                    .to_string()
                    .eq(&"Device is not connected with internet ".to_string())
                {
                    println!("Internet disconnect ");
                    return HttpResponse::BadRequest()
                        .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

                if status
                    .to_owned()
                    .to_string()
                    .eq(&"Payment acccept".to_string())
                {
                    println!("Payment Accepted ");
                    println!(
                        "Result ready {:?} ",
                        status.to_owned().to_string().eq(&"Payment acccept")
                    );
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

                let _ = match direct_gateway(fees).await {
                    Ok(_) => {
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
                    Err(e) => {
                        eprintln!("Error {:?}", e);
                        return HttpResponse::BadRequest()
                            .body(hbr.render("music_error", &RequestError {}).unwrap());
                    }
                };
            }
        }
    }

    // there may be possible that gpt access is not working properly or bad formatting then throw error
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

    if let Some(itv) = imovies.imdb_season(client).await {
        let _ = imovies.tv_shows(itv).await;

        let client = match gatekeeper::mongodb_client().await {
            Ok(list) => list,
            Err(e) => panic!("{:?}", e),
        };

        let db = client.database(music::MUSIC_RECORD);
        let fees: u64 = 100;

        unsafe {
            let nodeless = INodeless::new(
                fees,
                "".to_owned().to_string(),
                fees as f64,
                "user preference tv season".to_owned().to_string(),
                ME.to_owned().to_string(),
                lightnode_net::TransactionStatus::Pending,
                "".to_string(),
            );

            let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
            if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string()){

                println!("Nodeless Bitcoin Gateway down");
                return HttpResponse::BadRequest()
                        .body(hbr.render("music_error", &RequestError {}).unwrap());
            }

            if status
                .to_owned()
                .to_string()
                .eq(&"Device is not connected with internet ".to_string())
            {
                println!("Internet disconnect ");
                return HttpResponse::BadRequest()
                    .body(hbr.render("music_error", &RequestError {}).unwrap());
            }

            if status.to_owned().to_string().eq(&"Payment acccept") {
                println!("Payment Accepted ");
                println!(
                    "Result ready {:?} ",
                    status.to_owned().to_string().eq(&"Payment acccept")
                );

                return HttpResponse::Ok().body(
                    hbr.render(
                        "tv",
                        &MovieRecomend {
                            title: imovies.name.to_owned(),
                            genre_0: imovies.genre[0].to_owned(),
                            genre_1: imovies.genre[1].to_owned(),
                            genre_2: imovies.genre[2].to_owned(),
                            release: imovies.release.to_owned().to_string(),
                            content: imovies.adult.to_owned(),
                            watch_min: imovies.watch_min.to_owned() as i64,
                            official: imovies.official.to_owned(),
                        },
                    )
                    .unwrap(),
                );
            }

            let _ = match direct_gateway(fees).await {
                Ok(_) => {
                    return HttpResponse::Ok().body(
                        hbr.render(
                            "tv",
                            &MovieRecomend {
                                title: imovies.name.to_owned(),
                                genre_0: imovies.genre[0].to_owned(),
                                genre_1: imovies.genre[1].to_owned(),
                                genre_2: imovies.genre[2].to_owned(),
                                release: imovies.release.to_owned().to_string(),
                                content: imovies.adult.to_owned(),
                                watch_min: imovies.watch_min.to_owned() as i64,
                                official: imovies.official.to_owned(),
                            },
                        )
                        .unwrap(),
                    );
                }
                Err(e) => {
                    eprintln!("Error {:?}", e);
                    return HttpResponse::BadRequest()
                        .body(hbr.render("music_error", &RequestError {}).unwrap());
                }
            };
        }
    }

    println!("Unfortunately Movie Title is not found");
    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

// 16 search_epic => post

#[post("/user/itvshows/epic/{search}")]
async fn search_epic(
    form: web::Form<EpisodeSearch>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // which episode of a season you like to watch this weekend? such as ["Run"].
    let qsearch = &form.name;

    let client = MovieRate::imdb_client().await;

    let mut genre: Vec<Emotionfilter> = Vec::<Emotionfilter>::new();

    genre.push(Emotionfilter::None);

    // get season title like "Designated Surivor"
    if let Some(query) = SEARCHEPIC.get() {
        // get season release
        if let Some(year) = SEASONRELEASE.get() {
            // convert rellease year into numeric format
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
            let bank = imovies.get_episode(client).await;

            let (steps, flag) = imovies
                .get_episode_name(bank.to_owned(), qsearch.to_owned().to_string())
                .await;
            let show_label = imovies
                .get_episode_label(bank.to_owned(), steps, qsearch.to_owned().to_string())
                .await;
            let show_epic = imovies
                .get_episode_epic(bank.to_owned(), steps, qsearch.to_owned().to_string())
                .await;
            let show_watch = match imovies
                .get_episode_watch(bank.to_owned(), steps, qsearch.to_owned().to_string())
                .await
            {
                Some(min) => min,
                None => panic!("Error report"),
            };

            // get season id for the next steps. Once all data should collect then validate

            let show_id = imovies
                .get_episode_id(bank.to_owned(), steps, qsearch.to_owned().to_string())
                .await;

            // check whether all conditions meet
            if (flag == true && show_label != 5000)
                && (show_epic != 5000 && show_watch != 5000)
                && (show_id != 5000)
            {
                return HttpResponse::Ok().body(
                    hbr.render(
                        "epic",
                        &Recorded {
                            name: qsearch.to_owned().to_string(),
                            episode: show_label,
                            seasons: show_epic,
                            watch: show_watch,
                            link: ("https://www.imdb.com/title/tt".to_string()
                                + &show_id.to_owned().to_string()
                                + &"/mediaindex/?ref_=tt_mv_sm".to_string()),
                        },
                    )
                    .unwrap(),
                );
            }
        }
    }

    // due to poor formatted input app might be crash
    println!("Unfortunately Show Title is not found");
    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

// 17. search_artist => post
#[post("/user/library/{search}/{artist}")]
async fn search_artist(
    form: web::Form<SearchArtist>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // search for artist ["Akon"]
    let asearch = &form.name;

    let mut art = Vec::<String>::new();
    art.push(asearch.to_owned().to_string());

    unsafe {
        // validate user session
        let expire = gatekeeper::login_expire(ME);

        if expire {
            println!("Make sure you have provide correct information or session expired. ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
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
            0.0,
        );

        let client = match record.create_mongo_connection().await {
            Ok(list) => list,
            Err(e) => panic!("{:?}", e),
        };

        let db = client.database(music::MUSIC_RECORD);

        // check whether any [Akon] record exist in our record
        let stream_record = record.get_song_from_playlist_through_artist(db).await;

        // check whether artist name present in our database, if not then retry again

        if stream_record.len().gt(&0)
            && stream_record[stream_record.len() - 1]
                .song_name
                .to_owned()
                .eq(&"")
        {
            println!("This artist don't have any content on our platform.. ");
            return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
        }

        let mut iterate = stream_record.to_owned().into_iter();

        let mut record = Vec::<music::MusicRecord>::new();

        // there may be possible that [Akon] return artist fan page instead of music because no such record

        for music in iterate.by_ref() {
            if music.song_name.to_owned().eq(&"") {
                continue;
            }

            // otherwise hold whole song record
            record.push(music);
        }

        let mut content = pinata_content::Content::new(
            ME.to_string(),
            "".to_string(),
            "".to_string(),
            record[record.len() - 1].song_name.to_owned().to_string(),
            pinata_content::genre_to_emotions(
                stream_record[record.len() - 1].genre.to_owned().to_string(),
            ),
            false,
            0,
            0,
        );

        let client = match gatekeeper::mongodb_client().await {
            Ok(list) => list,
            Err(e) => panic!("{:?}", e),
        };

        let db = client.database(music::MUSIC_RECORD);

        // complete the process

        let playlist_song = content.get_playlist_by_song(db).await;

        return HttpResponse::Ok().body(
            hbr.render(
                "search",
                &SongEngine {
                    pmusic_artist: record[record.len() - 1].artist[0].to_owned(),
                    pmusic_compose: record[record.len() - 1].compose.to_owned(),
                    pmusic_genre: record[record.len() - 1].genre.to_owned(),
                    pmusic_ilink: playlist_song.cid_icontent.to_owned(),
                    pmusic_lyric: record[record.len() - 1].lyrics.to_owned(),
                    session: ME.to_string(),
                    name: record[record.len() - 1].song_name.to_owned(),
                    pmusic_mlink: playlist_song.cid_mcontent.to_owned(),
                    pnumic_production: record[record.len() - 1].studio_name.to_owned(),
                    favourite: playlist_song.like.to_owned(),
                    favourite_count: playlist_song.like_count.to_owned(),
                    played: playlist_song.play_count.to_owned(),
                    emotion: playlist_song.emotion.to_owned(),
                    comment: playlist_song.comment.to_owned(),
                    comment_like_count: playlist_song.comment_like_count.to_owned(),
                    comment_likes: playlist_song.comment_likes.to_owned(),
                    user_comments: playlist_song.followers_comments.to_owned(),
                },
            )
            .unwrap(),
        );
    }
}

// 19. search_emotion => post
#[post("/user/library/{search}/{music}/{emotion}")]
async fn search_emotion(
    form: web::Form<SearchEmotion>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    let emo = &form.name;

    // warning : search emotion functionality allow you to listen base on your emotion. There maybe possible user are depressed then no song will be played;

    if emo.to_owned().eq(&"Depressed") || emo.to_owned().eq(&"Sucide") {
        println!("Please visit doctor , medicate & listen light hearted songs");
        return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
    }

    unsafe {
        // validate user session
        let expire = gatekeeper::login_expire(ME);

        if expire {
            println!("Make sure you have provide correct information or session expired. ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
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

        let records = content
            .get_playlist_through_beat(db, emo.to_owned().to_string())
            .await;

        // if record return 0 then it throw error because no record will return against query...
        if records.len().eq(&0) {
            println!(" This emotion has no such playlist , hopefully next time .. ");
            return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
        }

        let mut list = Playlist {
            song: Vec::<String>::new(),
            session: Vec::<String>::new(),
            cid_icontent: Vec::<String>::new(),
            like: Vec::<bool>::new(),
            cid_mcontent: Vec::<String>::new(),
            like_count: Vec::<i64>::new(),
            emotion: Vec::<pinata_content::Emotionfilter>::new(),
            comment: Vec::<String>::new(),
            comment_like: Vec::<_>::new(),
            comment_like_count: Vec::<i64>::new(),
            followers_comments: Vec::<i64>::new(),
            song_count: 0,
        };

        let mut it = records.to_owned().into_iter();

        // data filter process initate
        for data in it.by_ref() {
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

        // active payment gateway for further transactions
        let client = match gatekeeper::mongodb_client().await {
            Ok(list) => list,
            Err(e) => panic!("{:?}", e),
        };

        let db = client.database(music::MUSIC_RECORD);
        let fees: u64 = 300;

        let nodeless = INodeless::new(
            fees,
            "".to_owned().to_string(),
            fees as f64,
            "search your emotion in our playlist... it's too hard "
                .to_owned()
                .to_string(),
            ME.to_owned().to_string(),
            lightnode_net::TransactionStatus::Pending,
            "".to_string(),
        );

        let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
        if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string()){

            println!("Nodeless Bitcoin Gateway down");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }

        if status
            .to_owned()
            .to_string()
            .eq(&"Device is not connected with internet ".to_string())
        {
            println!("Internet disconnect ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }

        if status.to_owned().to_string().eq(&"Payment acccept") {
            println!("Payment Accepted ");
            println!(
                "Result ready {:?} ",
                status.to_owned().to_string().eq(&"Payment acccept")
            );
            return HttpResponse::Ok().body(hbr.render("emotions", &list).unwrap());
        }

        let _ = match direct_gateway(fees).await {
            Ok(_) => {
                return HttpResponse::Ok().body(hbr.render("emotions", &list).unwrap());
            }
            Err(e) => {
                eprintln!("Error {:?}", e);
                return HttpResponse::BadRequest()
                    .body(hbr.render("music_error", &RequestError {}).unwrap());
            }
        };
    }
}

// 20. virtual book => get

#[get("/user/library/books")]
async fn virtual_book() -> impl Responder {
    NamedFile::open_async("./static/add_books.html").await
}

// 21. add book => post
#[post("/user/library/books/{add}/{book}/{publish}")]
async fn add_virtual_book(
    form: web::Form<VirtualBook>,
    hbr: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // input parse

    let title = &form.name;
    let author = &form.author;
    let pages = &form.pages;
    let description = &form.description;
    let isbn = &form.isbn;
    let publisher = &form.publisher;

    unsafe {
        // validate user session
        let expire = gatekeeper::login_expire(ME);

        if expire {
            println!(" User session expire {:?}", expire);
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    }

    // check whether title return empty or not . if condition is true (empty) then throw error.

    if title.to_owned().to_string().eq(&"") {
        println!("Tile should not be empty ");
        return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
    }

    // check whether author return empty or not . if condition is true (empty) then throw error.

    if author.to_owned().to_string().eq(&"") {
        println!("Author should not be empty ");
        return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
    }

    // check whether isbn return empty or not . if condition is true (empty) then throw error.
    if isbn.to_owned().to_string().eq(&"") {
        println!("ISBN should not be empty ");
        return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
    }

    // check whether description return empty or not . if condition is true (empty) then throw error.

    if description.to_owned().to_string().eq(&"") {
        println!("Description should not be empty ");
        return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
    }

    // check whether publisher return empty or not . if condition is true (empty) then throw error.
    if publisher.to_owned().to_string().eq(&"") {
        println!("Publisher should not be empty ");
        return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
    }

    // check whether pages return 0 or negative value . if condition is true (empty) then throw error.
    if pages.to_owned().eq(&0) && pages.to_owned().is_negative() {
        println!("Pages should not be 0 or negative ");
        return HttpResponse::Ok().body(hbr.render("collection", &Homepage {}).unwrap());
    }

    let mut instance = ipfs_net::IpfsBucket::new(title.to_owned().to_string());
    let path = instance.get_file_path();

    if path.to_owned().to_string().eq(&"") {
        // error report no such file exist in your machine download directory
        println!(
            "Book is not exist in download directory {:?} ",
            path.to_owned().to_string().eq(&"")
        );
        return HttpResponse::BadRequest()
            .body(hbr.render("music_error", &RequestError {}).unwrap());
    }

    let client = instance.pinta_client();
    let publish_hash = instance.publish_book(client, path.to_owned()).await;

    let mongoclient = pinata_ipfs::ipfs_net::Books::mongo_init().await;

    let hash = publish_hash.unwrap();

    let mut books = pinata_ipfs::ipfs_net::Books::new(
        title.to_string(),
        author.to_string(),
        publisher.to_string(),
        *pages as u16,
        description.to_string(),
        hash.ipfs_hash.to_string(),
    );

    let db = books.access_credentials(mongoclient.to_owned());

    unsafe {
        books.set_session(ME.to_owned().to_string());
    }

    let book_status = match books.create_book_doc(db).await {
        Ok(status) => status,
        Err(e) => {
            println!("Error {:?} ", e);
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    };

    if book_status.to_owned().to_string().eq(&"") {
        // active payment gateway for further transactions

        let client = match gatekeeper::mongodb_client().await {
            Ok(list) => list,
            Err(e) => panic!("{:?}", e),
        };

        let db = client.database(music::MUSIC_RECORD);
        let fees: u64 = books.on_self() as u64;

        let nodeless = INodeless::new(
            fees,
            "".to_owned().to_string(),
            fees as f64,
            "reader borrow masterpiece and pay for alchemy"
                .to_owned()
                .to_string(),
            books.get_session().await,
            lightnode_net::TransactionStatus::Pending,
            "".to_string(),
        );

        let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
        if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address"){

            println!("Nodeless Bitcoin Gateway down");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }

        if status
            .to_owned()
            .to_string()
            .eq(&"Device is not connected with internet ")
        {
            println!("Internet disconnect ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }

        if status.to_owned().to_string().eq(&"Payment acccept") {
            println!("Payment Accepted ");
            println!(
                "Result ready {:?} ",
                status.to_owned().to_string().eq(&"Payment acccept")
            );

            return HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap());
        }

        let _ = match direct_gateway(fees).await {
            Ok(_) => {
                return HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap());
            }

            Err(e) => {
                eprintln!("Error {:?}", e);
                return HttpResponse::BadRequest()
                    .body(hbr.render("music_error", &RequestError {}).unwrap());
            }
        };
    }

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

#[get("/user/sociallink/profile/edit")]
async fn edit() -> impl Responder {
    NamedFile::open_async("./static/profile.html").await
}

#[post("/user/sociallink/profile/edit/{your}")]
async fn details(form: web::Form<EditAccount>, hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    let name = &form.name;
    let lastname = &form.lastname;
    let city = &form.city;
    let country = &form.country;
    let degree = &form.degree;
    let baddress = &form.bitcoin;
    let work = &form.work;
    let company = &form.cname;
    let university = &form.sname;

    let address = &form.address;
    let fblink = &form.fblink;
    let instalink = &form.instalink;
    let xlink = &form.xlink;
    let youlink = &form.youlink;
    let phonenum = &form.phone;

    let change_avatar = &form.old_digitalverse;


    // check whether user login through user credentials.
    unsafe {
        let expire = gatekeeper::login_expire(ME.to_owned());

        if expire {
            println!("Make sure your account exist in our database ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    }

    if name.to_owned().to_string().eq(&"") {
        println!("Make sure your first name should not be empty ");
        return HttpResponse::BadRequest()
            .body(hbr.render("music_error", &RequestError {}).unwrap());
    }

    if lastname.to_owned().to_string().eq(&"") {
        println!("Make sure your last name should not be empty ");
        return HttpResponse::BadRequest()
            .body(hbr.render("music_error", &RequestError {}).unwrap());
    }

    if baddress.to_owned().to_string().eq(&"") {
        println!("Make sure your bitcoin address should not be empty ");
        return HttpResponse::BadRequest()
            .body(hbr.render("music_error", &RequestError {}).unwrap());
    }

    let mut my_info = auth::accounts::Info::new(
        name.to_owned().to_string(),
        lastname.to_owned().to_string(),
        university.to_owned().to_string(),
        degree.to_owned().to_string(),
        company.to_owned().to_string(),
        work.to_owned().to_string(),
        city.to_owned().to_string(),
        country.to_owned().to_string(),
        baddress.to_owned().to_string(),
        phonenum.to_owned().to_string(),
    );

    let mut avatar = Vec::<String>::new();
    avatar.push(change_avatar.to_owned());

    let mut home = Vec::<>::new();
    home.push(address.to_owned());

    my_info.address = home;
    my_info.fblink = fblink.to_owned();
    my_info.instalink = instalink.to_owned();
    my_info.xlink = xlink.to_owned();
    my_info.youlink = youlink.to_owned();
    my_info.new_digital = avatar;


    unsafe {
        my_info.set_session(ME.to_owned().to_string());
    }

    let mongo = my_info.mongo_init().await;
    let cred = my_info.access_credentials(mongo);
    let _record = match my_info.create_record_doc(cred.to_owned()).await {
        Ok(r) => r,
        Err(e) => {
            println!("Error {:?} ", e);
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    };

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

#[get("/user/sociallink/profile/search")]
async fn search() -> impl Responder {
    NamedFile::open_async("./static/search_person.html").await
}

#[post("/user/sociallink/profile/search/{your}/{friend}")]
async fn searching(form: web::Form<SearchParam>, hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    // check whether user login through user credentials.
    unsafe {
        let expire = gatekeeper::login_expire(ME.to_owned());

        if expire {
            println!("Make sure your account exist in our database ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    }

    // initalization & declaration
    let query = &form.query;
    let mut search_q: Vec<String> = Vec::<String>::new();
    let mut search_resp: Searched = Searched {
        name: search_q.to_owned(),
        counter: 0.to_string(),
        leads: 0.to_string(),
        follower: 0.to_string(),
        session: "".to_string(),
    };
    let mut tofind = auth::accounts::Info::new(
        query.to_owned().to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    );

    let minit = tofind.mongo_init().await;
    let access = tofind.access_credentials(minit);

    unsafe {
        tofind.set_session(ME.to_owned().to_string());
    }

    let resp = tofind
        .find_people_with_name(access.to_owned())
        .await
        .unwrap();


    let count = tofind.count_people(access.to_owned()).await.unwrap();


    if resp.is_empty() {
        
        return HttpResponse::BadRequest()
            .body(hbr.render("music_error", &RequestError {}).unwrap());
    }

    if resp.len().to_owned().ge(&1) {

        let mut iterate = resp.into_iter();

        for entity in iterate.by_ref() {
            
            search_q.push(entity.firstname + &entity.lastname);
            search_resp.session = entity.session.clone();

        }
    }

    search_resp.name = search_q.to_owned();
    search_resp.counter = count.to_owned().to_string();
    search_resp.leads = 0.to_owned().to_string();
    search_resp.follower = 0.to_owned().to_string();

    HttpResponse::Ok().body(hbr.render("person", &search_resp).unwrap())
}

#[post("/user/sociallink/profile/search/{your}/{friend}/{follow}")]
async fn new_follower(hbr: web::Data<Handlebars<'_>>) -> HttpResponse{

    // check whether user login through user credentials.
    unsafe {
        let expire = gatekeeper::login_expire(ME.to_owned());

        if expire {
            println!("Make sure your account exist in our database ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    }

    HttpResponse::Ok().body(hbr.render("home", &Homepage {}).unwrap())
}

#[post("/user/library/books/{find}/{book}/{record}/{accept}")]

async fn search_book(form: web::Form<Booksearch>, hbr: web::Data<Handlebars<'_>>) -> HttpResponse {
    let search_book = &form.bookname;

    if search_book.to_owned().to_string().eq(&"") {
        println!(
            "Empty keyword {:?} ",
            search_book.to_owned().to_string().eq(&"")
        );
        return HttpResponse::BadRequest()
            .body(hbr.render("music_error", &RequestError {}).unwrap());
    }

    let mut books = pinata_ipfs::ipfs_net::Books::new(
        search_book.to_owned().to_string(),
        "".to_string(),
        "".to_string(),
        0 as u16,
        "".to_string(),
        "".to_string(),
    );
    let mongoclient = pinata_ipfs::ipfs_net::Books::mongo_init().await;

    let db = books.access_credentials(mongoclient.to_owned());

    unsafe {
        books.set_session(ME.to_owned().to_string());
    }

    let record = match books.find_book_for_me(db.to_owned()).await {
        Ok(r) => r,
        Err(e) => {
            println!("Error {:?}", e);
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    };

    let client = match gatekeeper::mongodb_client().await {
        Ok(list) => list,
        Err(e) => panic!("{:?}", e),
    };

    let db = client.database(music::MUSIC_RECORD);
    let fees: u64 = books.on_self() as u64;

    let nodeless = INodeless::new(
        fees,
        "".to_owned().to_string(),
        fees as f64,
        "reader borrow masterpiece and pay for alchemy"
            .to_owned()
            .to_string(),
        books.get_session().await,
        lightnode_net::TransactionStatus::Pending,
        "".to_string(),
    );

    let status = payment_gateway(nodeless, db.to_owned()).await.unwrap();
    if status.to_owned().to_string().eq(&"Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address"){

                    println!("Nodeless Bitcoin Gateway down");
                    return HttpResponse::BadRequest()
                                .body(hbr.render("music_error", &RequestError {}).unwrap());
                }

    if status
        .to_owned()
        .to_string()
        .eq(&"Device is not connected with internet ")
    {
        println!("Internet disconnect ");
        return HttpResponse::BadRequest()
            .body(hbr.render("music_error", &RequestError {}).unwrap());
    }

    if status.to_owned().to_string().eq(&"Payment acccept") {
        println!("Payment Accepted ");
        println!(
            "Result ready {:?} ",
            status.to_owned().to_string().eq(&"Payment acccept")
        );

        return HttpResponse::Ok().body(
            hbr.render(
                "book",
                &GetBook {
                    name: record.book.to_owned().to_string(),
                    session: record.coonect.session.to_string(),
                    author: record.author.to_owned().to_string(),
                    publisher: record.publisher.to_owned().to_string(),
                    ipfs_link: "https://beige-aggressive-bird-900.mypinata.cloud/ipfs/".to_owned()
                        + &record.ipfs_link.to_owned().to_string(),
                    description: record.description.to_owned().to_string(),
                    page: record.page.to_owned().to_string(),
                },
            )
            .unwrap(),
        );
    }

    let _ = match direct_gateway(fees).await {
        Ok(_) => {
            return HttpResponse::Ok().body(
                hbr.render(
                    "book",
                    &GetBook {
                        name: record.book.to_owned().to_string(),
                        session: record.coonect.session.to_string(),
                        author: record.author.to_owned().to_string(),
                        publisher: record.publisher.to_owned().to_string(),
                        ipfs_link: "https://beige-aggressive-bird-900.mypinata.cloud/ipfs/"
                            .to_owned()
                            + &record.ipfs_link.to_owned().to_string(),
                        description: record.description.to_owned().to_string(),
                        page: record.page.to_owned().to_string(),
                    },
                )
                .unwrap(),
            );
        }
        Err(e) => {
            eprintln!("Error {:?}", e);
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    };
}


#[get("/user/sociallink/profile/discover")]
async fn discover() -> impl Responder{

    NamedFile::open_async("./static/v_profile.html").await

}

#[post("/user/sociallink/profile/discover/{name}")]
async fn discover_proximity(form: web::Form<Discover>, hbr: web::Data<Handlebars<'_>>) -> HttpResponse{

    let people = &form.discover;

    // check whether user login through user credentials.
    unsafe {
        let expire = gatekeeper::login_expire(ME.to_owned());

        if expire {
            println!("Make sure your account exist in our database ");
            return HttpResponse::BadRequest()
                .body(hbr.render("music_error", &RequestError {}).unwrap());
        }
    }

    let mut tofind = auth::accounts::Info::new(
        people.to_owned().to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    );

    let minit = tofind.mongo_init().await;
    let access = tofind.access_credentials(minit);

    unsafe {
        tofind.set_session(ME.to_owned().to_string());
    }

    let findperson = tofind.getaccount(access.to_owned()).await.unwrap();

    return HttpResponse::Ok().body(hbr.render("v_profile", &DiscoverPersonality {
        firstname : findperson.firstname.to_owned(),
        lastname : findperson.lastname.to_owned(),
        session : findperson.session.to_owned(),
        degree : findperson.degree.to_owned(),
        insitute : findperson.institute.to_owned(),
        address : findperson.address.to_owned(),
        workplace : findperson.workplace.to_owned(),
        career : findperson.work.to_owned(),
        bitcoinwallet : findperson.bitcoinaddr.to_owned(),
        fblink : findperson.fblink.to_owned(),
        xlink : findperson.xlink.to_owned(),
        youlink : findperson.youlink.to_owned(),
        instalink : findperson.instalink.to_owned(),
        new_digital : findperson.new_digital.to_owned(),
        city : findperson.city.to_owned(),
        country : findperson.country.to_owned(),
        avatar : findperson.new_digital.to_owned(),
        phone : findperson.phonenum.to_owned(),
    }).unwrap());
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // These lines allow to read secrets that is require to complete the process.
    dotenv().ok();
    let _token = ENV_TOKEN.set(
        env::var("OPENAI_API_KEY")
            .expect("token is not provided")
            .to_string(),
    );

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
            .service(edit)
            .service(details)
            .service(search)
            .service(searching)
            .service(shows)
            .service(search_shows)
            .service(search_epic)
            .service(search_artist)
            .service(search_emotion)
            .service(virtual_book)
            .service(add_virtual_book)
            .service(search_book)
            .service(discover)
            .service(discover_proximity)
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

pub async fn payment_gateway(mut nodeless: INodeless, db: Database) -> std::io::Result<String> {
    let accept: bool = false;

    let node = nodeless.create_nodeless_client().await;

    let status = node.to_owned().get_server_status().await;

    if let Ok(digital_store) = nodeless.connect_with_store(&node.to_owned()).await {
        if digital_store.name.is_empty() {
            println!("Make sure you connect with internet {:?} ", status);
            return Ok("Device is not connected with internet ".to_string());
        }

        let _ledger = nodeless.from_txs(db.to_owned()).await;

        if let Ok(block) = nodeless.lightnode_store_inovice(&node.to_owned()).await {
            let data = block.id.unwrap();
            nodeless.lid = data.to_owned();

            if let Some(email) = EMAIL.get() {
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

                    return Ok("Payment acccept".to_string());
                } else {
                    nodeless.status = lightnode_net::TransactionStatus::Expire;
                    nodeless.remaining = nodeless.amount as f64;

                    let _ = nodeless.update_tnx(db.to_owned()).await;

                    println!(
                        "Sorry Gateway has closed and kindly retry this operation {:?}",
                        tx[0]
                    );
                    println!("visit https://nodeless.io/app/stores/dashboard/e1be7458-9364-4f40-8de0-22a3d5af8db5/ for further information");
                    panic!(
                        "Payment gateway has closed retry this operation {:?}",
                        nodeless.status
                    );
                }
            }
        }
    }

    Ok("Sorry ! Nodeless Bitcoin Gateway can not accept your transaction for this time. Please use bitcoin address".to_string())
}

#[derive(Debug)]
enum BitcoinNetworkErrorReport {
    EmptyBitAddress,
    DuplicateBitAddress,
    InvalidBitAddress,
    TxFail,
    None,
}

async fn direct_gateway(fees: u64) -> Result<(), BitcoinNetworkErrorReport> {
    let addr = MY_BITCOIN_ADDR.get().unwrap();

    if addr.to_owned().to_string().eq(&"") {
        println!("Error your don't have bitcoin address");
        return Err(BitcoinNetworkErrorReport::EmptyBitAddress);
    } else {
        let mut bitpay = l2net::bitpayee::Bitenigma::new(addr.to_owned().to_string(), fees);
        let addr_val = bitpay.address_valid().unwrap();

        match addr_val {
            l2net::bitpayee::BitenigmaError::EmptyBitAddress(
                "This user have not provide bitcoin address",
            ) => {
                println!("Error your don't have bitcoin address");
                return Err(BitcoinNetworkErrorReport::EmptyBitAddress);
            }

            l2net::bitpayee::BitenigmaError::DuplicateAddress("This address is not Allowed ") => {
                println!("Error this address already register by someone");
                return Err(BitcoinNetworkErrorReport::DuplicateBitAddress);
            }

            l2net::bitpayee::BitenigmaError::InvalidAddressIssue(
                "This address is invalid adddress",
            ) => {
                println!("Error this address is not valid address");
                return Err(BitcoinNetworkErrorReport::InvalidBitAddress);
            }

            l2net::bitpayee::BitenigmaError::None => {
                let sender = bitpay.pay_handshake().await;

                println!("Sender Inovice {:?}", sender);

                if bitpay.valid_sender(sender) {
                    println!("transaction process complete");
                } else {
                    println!("Error transaction failed");
                    return Err(BitcoinNetworkErrorReport::TxFail);
                }

                let receiver = bitpay.rece_handshake().await;

                println!("Receiver Inovice {:?}", receiver);

                if bitpay.valid_receiver(receiver) {
                    println!("transaction process complete");
                } else {
                    println!("Error transaction failed");
                    return Err(BitcoinNetworkErrorReport::TxFail);
                }
            }

            _ => {
                todo!();
            }
        }
        Ok(())
    }
}
