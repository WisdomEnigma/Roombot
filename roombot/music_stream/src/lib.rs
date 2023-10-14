#[warn(unused_imports)]

pub mod music{


    use mongodb::{Client, options::{ClientOptions,FindOptions}, bson::doc, results::{InsertOneResult, InsertManyResult}};
    use futures_util::{stream::TryStreamExt, future::ok};
    use serde::{Deserialize, Serialize};



    pub static MUSIC_RECORD : & str = "Artists_Record";
    static Song_DB : &str = "songs";
    

    
    #[derive(Debug,Deserialize, Serialize, Clone)]
    pub struct MusicRecord{
    
        pub song_name : String,
        pub artist : Vec::<String>,
        pub cover_image : String,
        // pub song_status : PintaStatus,
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
    
    pub enum PintaStatus{
        Pin,
        Unpin,
    }
    
    
    
    pub fn new_beat(song : String, oartist : Vec::<String>, img : String, addr : String , date : String, lyrics_artist : String, studio : String, genre : String, compose : String, website: String, collobarate : String, royalty : bool, lightnode : bool, asset : bool, research : bool, ownership : bool, email : String, id : String, deposite : f64) -> MusicRecord{
    
        MusicRecord { 
            song_name: song, 
            artist : oartist, 
            cover_image: img, 
            // song_status: status, 
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

    impl MusicRecord{

        pub async fn create_mongo_connection(&mut self) -> std::io::Result<Client>{


           let client = match mongodb_client().await{
                Ok(client) => client,
                Err(e) => panic!("New client object error {:?}", e),
            };      
                       
            Ok(client)
        }

        pub async fn create_collection(&mut self, db : mongodb::Database) -> std::io::Result<()> {

          let collects = db.collection::<MusicRecord>(Song_DB);
          
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
            let collection = database.collection::<MusicRecord>(Song_DB);
            
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

        pub async fn get_song_name_from_playlist(&mut self, db : mongodb::Database) -> String{

            
            let result = self.find_with_song(db).await;

            if !self.matches(result.to_owned()){

                    return "".to_string();
            }

            result
        }   

        pub async fn find_song(&mut self, db : mongodb::Database) -> std::io::Result<MusicRecord>{

            let collection = db.collection::<MusicRecord>(Song_DB);
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




pub mod pinata_content{
    use std::panic;

    use mongodb::{options::{FindOptions, FindOneAndUpdateOptions}, bson::doc, results::{InsertOneResult, InsertManyResult}, Database};
    use futures_util::{stream::TryStreamExt};
    use serde::{Deserialize, Serialize};

    static COLLECTION : &str = "playlist";

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Content{

        pub session : String,
        pub cid_icontent : String, // images
        pub cid_mcontent : String, // music
        pub song : String,
        
        
        pub like : bool,        // user likes
        pub like_count : i64,   // user vote
        pub play_count : i64,  // music play
        pub emotion :   Emotionfilter  // mood of user

    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Emotionfilter{

        Dancing,
        Passion,
        Love,
        Sad,
        None,
        Mixed,
    }

    impl Content{

        pub fn new(id : String, imghash : String, audiohash : String, song : String, views : Emotionfilter, like : bool, like_count: i64, play : i64) -> Self{
            Self { session: id.to_string(), cid_icontent: imghash.to_string(), cid_mcontent: audiohash.to_string(), song : song.to_string(), like, like_count, play_count : play, emotion : views  }
        }

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
                    },
                ];

                let _ = collect.insert_many(doc, None).await;
            }
            
            
            Ok(())            
        }

    
        async fn find_playlist_with_session(&mut self, db : Database) -> std::io::Result<Content>{

        
            let collect = db.collection::<Content>(COLLECTION);

            let mut playlist : Content = Content { session: "".to_string(), cid_icontent: "".to_string(), cid_mcontent: "".to_string(), song : "".to_string(), like : false, like_count: 0, play_count : 0, emotion : Emotionfilter::None};

            
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

        pub async fn get_playlist(&mut self, db : Database) -> Content{

            

            let mut playlist = Content{
                session : "".to_string(), 
                cid_icontent : "".to_string(),
                cid_mcontent : "".to_string(),
                song : "".to_string(),
                like_count : 0,
                like : false,
                play_count : 0,
                emotion : Emotionfilter::None
            };

            
            // let collect = db.collection::<Content>(COLLECTION);
            let query = self.find_playlist_with_session(db).await;
            if let Ok(content) = query{

                playlist = content;
            }

            playlist 
        }

        async fn find_playlist_with_song(&mut self, db :Database) -> std::io::Result<Content>{

            let collect = db.collection::<Content>(COLLECTION);

            let mut playlist : Content = Content { session: "".to_string(), cid_icontent: "".to_string(), cid_mcontent: "".to_string(), song : "".to_string(), like : false, like_count : 0, play_count : 0, emotion : Emotionfilter::None};

            

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

        pub async fn get_playlist_by_song(&mut self, db : Database) -> Content{

            let mut playlist = Content{
                session : ("".to_string()), 
                cid_icontent : ("".to_string()),
                cid_mcontent : ("".to_string()),
                song : ("".to_string()),
                like : false,
                like_count : 0,
                play_count : 0,
                emotion : Emotionfilter::None
            };

            
            
            let query = self.find_playlist_with_song(db).await;
            if let Ok(content) = query{

                playlist = content;
            }

            playlist            
        }

        pub async fn update_song_info(&mut self, db : Database) -> Content {
            
            let collect = db.collection::<Content>(COLLECTION);
            
                
                let filter = doc!{ "song" : self.song.to_owned()};
                let update_doc = doc! {
                    "$set" : {
                        "like" : self.like,
                        "like_count" : self.like_count,
                        "play_count" : self.play_count,
                    },
                };

                let update_opts = FindOneAndUpdateOptions::builder().return_document(mongodb::options::ReturnDocument::After).build();
                if let Ok(result ) =  collect.find_one_and_update(filter, update_doc, update_opts).await{

                    if let Some(content) = result {

                        return content;
                    }
                }
            
            Content{
                session : "".to_string(),
                cid_icontent : "".to_string(),
                cid_mcontent : "".to_string(),
                song : "".to_string(),
                like : false,
                like_count : 0,
                play_count : 0,
                emotion : Emotionfilter::None,
            }
        }
    }

}
