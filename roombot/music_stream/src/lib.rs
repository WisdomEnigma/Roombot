#[warn(unused_imports)]

/// music module allow to store record about music stream this will help in content publishing , content update.
/// 
/// ********************************************
/// 
/// This codebase maintain under mozilla public licence , for further information read licence terms which is attached with project.


pub mod music{

    /// ****************************************
    // import
    use mongodb::{Client, options::{ClientOptions,FindOptions}, bson::doc, results::{InsertOneResult, InsertManyResult}};
    use futures_util::{stream::TryStreamExt, future::ok};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;



    /// Music Record is a public static reference.
    pub static MUSIC_RECORD : & str = "Artists_Record";
    static SONG_DB : &str = "songs";
    static mut AUDIO: Vec<HashMap<String, bool>> = Vec::new();
    


    /// Music Record derive by many traits. Music records have lots of fields. These fields operate various tasking regarding data transaction and persistance.
    
    #[derive(Debug,Deserialize, Serialize, Clone)]
    pub struct MusicRecord{
    
        pub song_name : String,
        pub artist : Vec::<String>,
        pub cover_image : String,
        light_node_addr  : String,
        pub release_date : String,
        pub lyrics : String,
        pub studio_name : String,
        pub genre : String,
        pub compose : String,
        studio_website : String,
        collobarate : String,
        royalty : bool,
        lightnode : bool,
        asset : bool,
        research : bool,
        ownership : bool,
        email : String,
        pub session : String,
        pub price : f64,
    }
    
    


    /// set audio hold song memory address of your disk. Song should pass hashmap becuase either specific song is play or not 
    pub fn set_audio(source: HashMap<String, bool>) {
        
        unsafe {
            AUDIO.push(source);
        }
    }
    
    /// get audio return song which user want to listen. Function return hashmap as well.
    pub fn get_audio() -> HashMap<String, bool> {
        
        let mut value: HashMap<String, bool> = HashMap::new();
        
        unsafe {
            for i in 0..AUDIO.len() {
                let key = match AUDIO.get(i) {
                    Some(k) => k,
                    None => panic!("Error reporting"),
                };
    
                value = key.clone();
            }
        }
    
        value
    }
    
    
    /// new beat return instance of Music record
    pub fn new_beat(song : String, oartist : Vec::<String>, img : String, addr : String , date : String, lyrics_artist : String, studio : String, genre : String, compose : String, website: String, collobarate : String, royalty : bool, lightnode : bool, asset : bool, research : bool, ownership : bool, email : String, id : String, deposite : f64) -> MusicRecord{
    
        MusicRecord { 
            song_name: song, 
            artist : oartist, 
            cover_image: img, 
            light_node_addr: addr, 
            release_date: date, 
            lyrics: lyrics_artist,
            studio_name: studio, 
            genre: genre, 
            compose: compose, 
            studio_website: website, 
            collobarate: collobarate, 
            royalty: royalty, 
            lightnode: lightnode, 
            asset: asset, 
            research: research, 
            ownership: ownership,
            email: email,
            session : id, 
            price : deposite,
        }
    }


    /// Music Record implementation are
    /// 
    /// create mongo connection
    /// create collection
    /// get_song_name_from_playlist
    /// find_song
    impl MusicRecord{


        /// mongo connection allow to create mongodb instance. if everything work fine.
        pub async fn create_mongo_connection(&mut self) -> std::io::Result<Client>{


           let client = match mongodb_client().await{
                Ok(client) => client,
                Err(e) => panic!("New client object error {:?}", e),
            };      
                       
            Ok(client)
        }

        /// create collection allow to create collection in mongodb
        pub async fn create_collection(&mut self, db : mongodb::Database) -> std::io::Result<()> {

          let collects = db.collection::<MusicRecord>(SONG_DB);
          
          let result = self.find_with_song(db).await;

          
          if !self.matches(result){

            let doc = vec![
                
                MusicRecord{
                    song_name : self.song_name.to_string(),
                    artist : self.artist.to_owned(), 
                    cover_image : self.cover_image.to_string(),
                    light_node_addr : self.light_node_addr.to_string(), 
                    release_date : self.release_date.to_string(), 
                    lyrics : self.lyrics.to_string(),
                    studio_name : self.studio_name.to_string(), 
                    genre : self.genre.to_string(), 
                    compose : self.compose.to_string(), 
                    studio_website : self.studio_website.to_string(), 
                    collobarate : self.collobarate.to_string(), 
                    royalty : self.royalty,
                    lightnode : self.lightnode, 
                    asset : self.asset, 
                    research : self.research,
                    ownership : self.ownership, 
                    email : self.email.to_string(),
                    session : self.session.to_string(),
                    price : self.price.to_owned(),
                },
                
            ];

            let _ = collects.insert_many(doc, None).await;

          }

          Ok(())
        }

        async fn find_with_song(&mut self, database : mongodb::Database) -> String {

            let mut query : String = "".to_string(); 
            let collection = database.collection::<MusicRecord>(SONG_DB);
            
            let filter = doc!{ "song_name" : self.song_name.to_owned()};

            let find_opts = FindOptions::builder().sort(doc!{ "song_name" : 1}).build();

            let mut cursor = collection.find(filter, find_opts).await.unwrap();
            
            while let Some(record) = cursor.try_next().await.unwrap(){

                

                if record.song_name == " "{
                    panic!("Unforuente query must be empty ");
                }

                if record.song_name != self.song_name{

                    panic!("No Data found ");
                }

                 query = record.song_name;
            }

            query
        }

        fn matches(&mut self, beat_2 : String) -> bool {

            if self.song_name.to_owned() != beat_2.to_owned(){ 
                return false; 
            }
            
            true
        }


        /// get song name from playlist is a search song by song name .... 
        pub async fn get_song_name_from_playlist(&mut self, db : mongodb::Database) -> String{

            
            let result = self.find_with_song(db).await;

            if !self.matches(result.to_owned()){

                    return "".to_string();
            }

            result
        }   


        /// find song feature allow to look up in playlist for us. However , it will return result of Music Record
        pub async fn find_song(&mut self, db : mongodb::Database) -> std::io::Result<MusicRecord>{

            let collection = db.collection::<MusicRecord>(SONG_DB);
            let mut song_class: MusicRecord = MusicRecord{
                song_name : "".to_string(), 
                artist : Vec::<String>::new(),
                cover_image : "".to_string(), 
                release_date :"".to_string(), 
                light_node_addr : "".to_string(),
                lyrics: "".to_string(),
                studio_name : "".to_string(),
                genre: "".to_string(),
                compose : "".to_string(),
                studio_website: "".to_string(), 
                collobarate : "".to_string(),
                royalty : false,
                lightnode: false,
                asset : false,
                research :false,
                ownership: false,
                email: "".to_string(), 
                session: "".to_string(), price : 0.0,};

                let filter = doc!{ "song_name" : self.song_name.to_owned()};
                let find_opts = FindOptions::builder().sort(doc!{ "song_name" : 1}).build();

                let mut cursor = collection.find(filter, find_opts).await.unwrap();
            
                while let Some(record) = cursor.try_next().await.unwrap(){

                    if record.song_name == ""{

                        panic!("Unforuente query must be empty ");
                    }

                    song_class = record;
                }

            Ok(song_class)
        }
        

        /// get song from playlist return whole record of a song. from genre to artists...
        pub async fn get_song_from_playlist(&mut self, db: mongodb::Database) -> MusicRecord{

            let mut song_class: MusicRecord = MusicRecord{
                song_name : "".to_string(), 
                artist : Vec::<String>::new(),
                cover_image : "".to_string(), 
                release_date :"".to_string(), 
                light_node_addr : "".to_string(),
                lyrics: "".to_string(),
                studio_name : "".to_string(),
                genre: "".to_string(),
                compose : "".to_string(),
                studio_website: "".to_string(), 
                collobarate : "".to_string(),
                royalty : false,
                lightnode: false,
                asset : false,
                research :false,
                ownership: false,
                email: "".to_string(), 
                session: "".to_string(), price : 0.0};
            
            let result_data = self.find_song(db).await;

            if let Ok(result) = result_data{
                
                
                if !self.matches(result.song_name.to_owned()){

                    return song_class;
                }
            
                song_class = result;
            }

            song_class
                
            }
    }
    
    
    
    
    async fn mongodb_client() -> Result<Client,mongodb::error::Error>{

        let client_opts = match ClientOptions::parse("mongodb+srv://enigmabot:nigkjv8emfgPpoeI@streambusiness.nkakl0h.mongodb.net/").await{
            Ok(options) => options,
            Err(e) => panic!("Error {:?}", e),
        };
        

        let client = Client::with_options(client_opts);
        client
    }

}


/// pinata_content is another module which is store pinata data for many purpose. Allow searching, update and retreive data.
/// content have many fields and all fields are open for public.
/// Another main feature this module provide is to categorize the song based on beat.

pub mod pinata_content{
    

    // imports 
    
    use std::panic;
    use mongodb::{options::{FindOptions, FindOneAndUpdateOptions}, bson::doc, results::{InsertOneResult, InsertManyResult}, Database};
    use futures_util::stream::TryStreamExt;
    use serde::{Deserialize, Serialize};


    // static reference
    static COLLECTION : &str = "playlist";


    /// Content is a public object and all fields are accessible outside the module.
    /// This object derive from many traits.
    ///   cid_mcontent hold refernce of music stream 
    ///    cid_icontent hold image reference
    /// 
    ///    like rating algorithm
    ///    like_count number of likes per song
    ///    play_count number of users listen
    /// 
    ///    [emotion filter] beat categorization
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Content{

        pub session : String,
        pub cid_icontent : String, // images
        pub cid_mcontent : String, // music
        pub song : String,
        
        
        pub like : bool,        // user likes
        pub like_count : i64,   // user vote
        pub play_count : i64,  // music play
        pub emotion :   Emotionfilter,  // mood of user
        pub comment : String, 
        pub comment_like_count : i64,
        pub comment_likes : bool,        
        pub followers_comments : i64,       // total comments on a song
    }

    /// Emotion Filter enumerate allow further definition. Classification of beats
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Emotionfilter{

        Dancing,
        Passion,
        Love,
        Sad,
        None,
        Mixed,
    }

    

    

    /// genre_to_emotion take song genre and return song classified song based on your mood or emotion.
    pub fn genre_to_emotions(genre: String) -> Emotionfilter {
        if genre.contains("rock")
            || genre.contains("Rock")
            || genre.contains("Pop rock")
            || genre.contains("pop rock")
            || genre.contains("classical music")
            || genre.contains("Classical music")
            || genre.contains("Blues")
            || genre.contains("blues")
        {
            return Emotionfilter::Sad;
        } else if genre.contains("Jazz")
            || genre.contains("jazz")
            || genre.contains("soul music")
            || genre.contains("Soul music")
        {
            return Emotionfilter::Love;
        } else if genre.contains("Rhythm and blues") || genre.contains("rhythm and blues") {
            return Emotionfilter::Passion;
        } else if genre.contains("Contemporary classical music")
            || genre.contains("contemporary classical music")
        {
            return Emotionfilter::Dancing;
        } else if genre.contains("Musical theatre")
            || genre.contains("musical theatre")
            || genre.contains("pop")
            || genre.contains("Pop")
        {
            return Emotionfilter::Love;
        } else if genre.contains("Alternative rock") || genre.contains("alternative rock") {
            return Emotionfilter::Mixed;
        } else {
            return Emotionfilter::None;
        }
    }


    

    impl Content{

        /// new allow to create instance of Content. 
        pub fn new(id : String, imghash : String, audiohash : String, song : String, views : Emotionfilter, like : bool, like_count: i64, play : i64) -> Self{
            Self { session: id.to_string(), cid_icontent: imghash.to_string(), cid_mcontent: audiohash.to_string(), song : song.to_string(), like, like_count, play_count : play, emotion : views, comment : "".to_string(),comment_like_count : 0, comment_likes : false, followers_comments : 0,  }
        }

        


        /// music collection collect information about songs such as artist, song name, music_refernce etc .. More information read pinata content module description.
        /// theremay be possible that song is not uploaded by artist then song is available for fans. 
        pub async fn music_collection(&mut self, db : Database) -> std::io::Result<()>{

            let collect = db.collection::<Content>(COLLECTION);

            let query = match self.find_playlist_with_session(db).await{

                Ok(query) => query,
                Err(e) => panic!("{:?}", e),
            };

            
            
            if self.cid_icontent != query.cid_icontent && self.cid_mcontent != query.cid_mcontent {

                let doc = vec![
                    Content{

                        session : self.session.to_string(),
                        cid_icontent : self.cid_icontent.to_string(),
                        cid_mcontent : self.cid_mcontent.to_string(),
                        song : self.song.to_string(),
                        like : self.like,
                        like_count : self.like_count,

                        play_count : self.play_count,
                        emotion : self.emotion.clone(),
                        comment : self.comment.clone(),
                        comment_like_count : self.comment_like_count,
                        comment_likes : self.comment_likes,
                        followers_comments : self.followers_comments,
                    },
                ];

                let _ = collect.insert_many(doc, None).await;
            }
            
            
            Ok(())            
        }

    
        // find playlist with session allow to search song 
        async fn find_playlist_with_session(&mut self, db : Database) -> std::io::Result<Content>{

        
            let collect = db.collection::<Content>(COLLECTION);

            let mut playlist : Content = Content { session: "".to_string(), cid_icontent: "".to_string(), cid_mcontent: "".to_string(), song : "".to_string(), like : false, like_count: 0, play_count : 0, emotion : Emotionfilter::None, comment : "".to_string(), comment_like_count : 0, comment_likes : false, followers_comments: 0};

            
            let filter = doc!{ "session" : self.session.to_owned()};
            let find_opts = FindOptions::builder().sort(doc!{ "session" : -1}).build();
            let mut cursor = collect.find(filter, find_opts).await.unwrap();
            
            while let Some(record) = cursor.try_next().await.unwrap(){

                if record.session == " "{
                    panic!("Unforuente query must be empty ");
                }

                if record.song == self.song{

                    playlist = record;
                    break;
                }

                 continue       
            }

            Ok(playlist)
        }


        // get playlist return all songs that exit in the platform.
        pub async fn get_playlist(&mut self, db : Database) -> Content{

            

            let mut playlist : Content = Content { session: "".to_string(), cid_icontent: "".to_string(), cid_mcontent: "".to_string(), song : "".to_string(), like : false, like_count: 0, play_count : 0, emotion : Emotionfilter::None, comment : "".to_string(), comment_like_count : 0, comment_likes : false, followers_comments: 0};

    
            let query = self.find_playlist_with_session(db).await;
            if let Ok(content) = query{

                playlist = content;
            }

            playlist 
        }

        // find playlist with song allow search song through song name.
        async fn find_playlist_with_song(&mut self, db :Database) -> std::io::Result<Content>{

            let collect = db.collection::<Content>(COLLECTION);

            let mut playlist : Content = Content { session: "".to_string(), cid_icontent: "".to_string(), cid_mcontent: "".to_string(), song : "".to_string(), like : false, like_count: 0, play_count : 0, emotion : Emotionfilter::None, comment : "".to_string(), comment_like_count : 0, comment_likes : false, followers_comments: 0};

            

            let filter = doc!{ "song" : self.song.to_owned()};
            let find_opts = FindOptions::builder().sort(doc!{ "song" : 1}).build();
            let mut cursor = collect.find(filter, find_opts).await.unwrap();

            
            while let Some(record) = cursor.try_next().await.unwrap(){

                
                if record.session == " "{
                    panic!("Unforuente query must be empty ");
                }

                playlist = record;        
            }

            Ok(playlist)            
        }

        // get playlist by song return song which you want to listen, if song exit in platform.
        pub async fn get_playlist_by_song(&mut self, db : Database) -> Content{

            let mut playlist : Content = Content { session: "".to_string(), cid_icontent: "".to_string(), cid_mcontent: "".to_string(), song : "".to_string(), like : false, like_count: 0, play_count : 0, emotion : Emotionfilter::None, comment : "".to_string(), comment_like_count : 0, comment_likes : false, followers_comments: 0};

            
            
            let query = self.find_playlist_with_song(db).await;
            if let Ok(content) = query{

                playlist = content;
            }

            playlist            
        }

        // update song information such as rating, listener_counter, play_Counter. 
        pub async fn update_song_info(&mut self, db : Database) -> Content {
            
            let collect = db.collection::<Content>(COLLECTION);

                            
                let filter = doc!{ "song" : self.song.to_owned()};
                let update_doc = doc! {
                    "$set" : {
                        "like" : self.like,
                        "like_count" : self.like_count,
                        "play_count" : self.play_count,
                        "comment" : self.comment.clone(),
                        "comment_like_count" : self.comment_like_count,
                        "comment_likes" : self.comment_likes,
                        "followers_comments" : self.followers_comments,
                    },
                };

                let update_opts = FindOneAndUpdateOptions::builder().return_document(mongodb::options::ReturnDocument::After).build();
                if let Ok(result ) =  collect.find_one_and_update(filter, update_doc, update_opts).await{

                    if let Some(content) = result {

                        return content;
                    }
                }
            
                Content { session: "".to_string(), cid_icontent: "".to_string(), cid_mcontent: "".to_string(), song : "".to_string(), like : false, like_count: 0, play_count : 0, emotion : Emotionfilter::None, comment : "".to_string(), comment_like_count : 0, comment_likes : false, followers_comments: 0}
        }
    }

}

