pub mod music{

    use mongodb::{Client, options::{ClientOptions,FindOptions}, bson::Document, bson::doc, results::{InsertOneResult, InsertManyResult}};
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
        session : String,
    }
    
    pub enum PintaStatus{
        Pin,
        Unpin,
    }
    
    
    
    pub fn new_beat(song : String, oartist : Vec::<String>, img : String, addr : String , date : String, lyrics_artist : String, studio : String, genre : String, compose : String, website: String, collobarate : String, royalty : bool, lightnode : bool, asset : bool, research : bool, ownership : bool, email : String, id : String) -> MusicRecord{
    
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
                },
                
            ];

            let _ = collects.insert_many(doc, None).await;

          }

          Ok(())
        }

        pub async fn find_with_song(&mut self, database : mongodb::Database) -> String {

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

pub mod Pinata_Content{
    use std::panic;

    use mongodb::{Client, options::{ClientOptions,FindOptions}, bson::Document, bson::doc, results::{InsertOneResult, InsertManyResult}, Database};
    use futures_util::{stream::TryStreamExt, future::ok};
    use serde::{Deserialize, Serialize};

    static COLLECTION : &str = "playlist";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Content{

        session : String,
        cid_icontent : String, // images
        cid_mcontent : String, // music
    }

    impl Content{

        pub fn new(id : String, imghash : String, audiohash : String) -> Self{
            Self { session: id.to_string(), cid_icontent: imghash.to_string(), cid_mcontent: audiohash.to_string() }
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
                    },
                ];

                let _ = collect.insert_many(doc, None).await;
            }
            
            
            Ok(())            
        }

    
        async fn find_playlist_with_session(&mut self, db : Database) -> std::io::Result<Content>{

        
            let collect = db.collection::<Content>(COLLECTION);

            let mut playlist : Content = Content { session: "".to_string(), cid_icontent: "".to_string(), cid_mcontent: "".to_string() };

            
            let filter = doc!{ "session" : self.session.to_owned()};
            let find_opts = FindOptions::builder().sort(doc!{ "session" : 1}).build();
            let mut cursor = collect.find(filter, find_opts).await.unwrap();

            
            while let Some(record) = cursor.try_next().await.unwrap(){

                
                if record.session == " "{
                    panic!("Unforuente query must be empty ");
                }

                playlist = record;        
            }

            Ok(playlist)
        }

        pub async fn get_playlist(&mut self, db : Database) -> Content{

            let mut playlist = Content{
                session : ("".to_string()), 
                cid_icontent : ("".to_string()),
                cid_mcontent : ("".to_string()),
            };

            
            // let collect = db.collection::<Content>(COLLECTION);
            let query = self.find_playlist_with_session(db).await;
            if let Ok(content) = query{

                playlist = content;
            }

            playlist 
        }
    }

}
