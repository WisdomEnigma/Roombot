pub mod music{

    use mongodb::{Client, options::{ClientOptions,FindOptions}, bson::Document, bson::doc, results::{InsertOneResult, InsertManyResult}};
    use futures_util::{stream::TryStreamExt, future::ok};
    use serde::{Deserialize, Serialize};



    pub static MUSIC_RECORD : & str = "Artists_Record";
    static SongDB : &str = "songs";

    
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
    }
    
    pub enum PintaStatus{
        Pin,
        Unpin,
    }
    
    
    
    pub fn new_beat(song : String, oartist : Vec::<String>, img : String, addr : String , date : String, lyrics_artist : String, studio : String, genre : String, compose : String, website: String, collobarate : String, royalty : bool, lightnode : bool, asset : bool, research : bool, ownership : bool, email : String) -> MusicRecord{
    
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
            email: email 
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

          let collects = db.collection::<MusicRecord>(SongDB);
          
          let result = self.find_with_song(self.song_name.to_string(), db).await;
          
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
                    
                },
                
            ];

            let _ = collects.insert_many(doc, None).await;

          }

          Ok(())
        }

        pub async fn find_with_song(&mut self, value : String, database : mongodb::Database) -> String {

            let mut query : String = "".to_string(); 
            let collection = database.collection::<MusicRecord>(SongDB);
            
            let filter = doc!{ "song_name" : value.to_owned()};

            let find_opts = FindOptions::builder().sort(doc!{ "song_name" : 1}).build();
            let mut cursor = collection.find(filter, find_opts).await.unwrap();

            
            while let Some(record) = cursor.try_next().await.unwrap(){

                if record.song_name == " "{
                    panic!("Unforuente query must be empty ");
                }

                if record.song_name != value{

                    panic!("No Data found ");
                }

                 query = record.song_name;
            }

            query
        }

        fn matches(&mut self, beat_2 : String) -> bool {

            println!("beat_1 = {:?}, beat_2 = {:?}", self.song_name.to_owned(), beat_2);
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


pub mod music_blob{


    use uplink_sys;
    use uplink_sys::UplinkAccessResult;
    use std::ffi::CString;
    
        pub fn connect_with_uplink() -> UplinkAccessResult {

            let satellite_address : CString = CString::new("12L9ZFwhzVpuEKMUNUqkaTLGzwY9G24tbiigLiXpmZWKwmcNDDs@eu1.storj.io:7777").expect("Address might be wrong");
            let apikey : CString  = CString::new("122VG4fJ9LA8BUQN55eQyfj5944VDam9Anbc95euDv1VfEL1QDkCJskmcfmABvBFZS4v5zGpSF8Ch44SynQoeRRh7WWwYYDnzdxPX1XoWv2kVsfhm8c3HMja59QjryphLiQLYnpP").expect("api key parse");
            let passphrase : CString = CString::new("word pluck pool device range current clinic blast submit approve fluid arrange").expect("passphrase expire");
            unsafe{

                uplink_sys::uplink_request_access_with_passphrase(
                    satellite_address.as_ptr() as *mut uplink_sys::uplink_const_char,
                    apikey.as_ptr() as *mut uplink_sys::uplink_const_char,
                    passphrase.as_ptr() as *mut uplink_sys::uplink_const_char,
                )

            }
             
    }
}