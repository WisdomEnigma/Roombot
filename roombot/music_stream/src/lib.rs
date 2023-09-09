pub mod music{

    use mongodb::{Client, options::ClientOptions, bson::Document, bson::doc, results::{InsertOneResult, InsertManyResult}};

    pub static MUSIC_RECORD : & str = "Artists_Record";
    pub struct MusicRecord{
    
        pub song_name : String,
        pub artist : Vec::<String>,
        pub cover_image : String,
        pub song_status : PintaStatus,
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
    
    
    pub fn new_beat(song : String, oartist : Vec::<String>, img : String, status : PintaStatus, addr : String , date : String, lyrics_artist : String, studio : String, genre : String, compose : String, website: String, collobarate : String, royalty : bool, lightnode : bool, asset : bool, research : bool, ownership : bool, email : String) -> MusicRecord{
    
        MusicRecord { 
            song_name: song, 
            artist : oartist, 
            cover_image: img, 
            song_status: status, 
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

        pub async fn create_collection(&mut self, db : mongodb::Database) -> Result<InsertManyResult, mongodb::error::Error> {

          let collects = db.collection::<Document>("songs");
          let doc = vec![
                doc! [
                    "song_name" : self.song_name.to_owned(), "artist" : self.artist[0].to_owned(), "cover_img" : self.cover_image.to_owned(),
                    "light_node_addr" : self.light_node_addr.to_owned(), "release_date" : self.release_date.to_owned(), "lyrics" : self.lyrics.to_owned(),
                    "studio_name" : self.studio_name.to_owned(), "genre" : self.genre.to_owned(), "compose" : self.compose.to_owned(), 
                    "studio_website" : self.studio_website.to_owned(), "collobarate" : self.collobarate.to_owned(), "royalty" : self.royalty,
                    "lightnode" : self.lightnode, "asset" : self.asset, "research" : self.research,
                    "ownership" : self.ownership, "email" : self.email.to_owned()
                ],
                
          ];

          collects.insert_many(doc, None).await

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
