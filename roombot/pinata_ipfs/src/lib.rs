///
/// All the changes made according to wisdomenigma rules & MPL Licence terms.
///
/// Redistribution, Commitment of work, Licence of Work, Intellectual Property & trademark.   
///
///
/// Contact us
///   github.com/WisdomEnigma                   wizdwarfs@gmail.com

pub mod ipinata{

    use std::{path::Path, path::PathBuf};
    use pinata_sdk::{PinataApi,PinByFile, PinnedObject, ApiError};
    use directories::UserDirs;
    

    
    /// Blob is a powerful object which require following incridents file, api, token & status. 
    pub struct Blob<'a>{

        file : &'a Path,  // file hold path
        api : &'a str,      // ipinata api
        token : &'a str,    // ipinata token
        status : FileStatus // status of file
    }


    /// File status meaning either file upload or remove from ipfs network.
    
    pub enum FileStatus{
        Pin,
        Unpin,
    }


    /// new blob object create pinata credentials for users.
    pub fn new_blob_object<'a>(file : &'a Path, operation : FileStatus) -> Blob<'a>{

        let (key, pass, _) = create_credentials();
        
        Blob{
            file : file,
            api : key,
            token : pass,
            status : operation
        }
    }    


    // create credentials is a private function which only accessible within module. This function return string literals as arguments.
    fn create_credentials<'a>() -> (&'a str, &'a str, &'a str){

        const KEY : &'static str = "2a5fcc53ad9c7e814daa";
        const SECRET : &'static str = "496a37d27f698a32f7161d993455643f055e47606af98bc2ddf2c19403f4cb49";
        const  CODE : &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiIzOTA5YWE1NC1kNmI2LTQxN2MtYmQxZC1mMDVlZDE5ZDhmNzIiLCJlbWFpbCI6ImFsaWRldmVsb3Blcjk1QGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaW5fcG9saWN5Ijp7InJlZ2lvbnMiOlt7ImlkIjoiTllDMSIsImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxfV0sInZlcnNpb24iOjF9LCJtZmFfZW5hYmxlZCI6ZmFsc2UsInN0YXR1cyI6IkFDVElWRSJ9LCJhdXRoZW50aWNhdGlvblR5cGUiOiJzY29wZWRLZXkiLCJzY29wZWRLZXlLZXkiOiIyYTVmY2M1M2FkOWM3ZTgxNGRhYSIsInNjb3BlZEtleVNlY3JldCI6IjQ5NmEzN2QyN2Y2OThhMzJmNzE2MWQ5OTM0NTU2NDNmMDU1ZTQ3NjA2YWY5OGJjMmRkZjJjMTk0MDNmNGNiNDkiLCJpYXQiOjE2OTMzODgzMTZ9.bj-XI0u01IU8Gov0mjxwnh2lYdL9ln1rXvCNS5dwWrI";
        (KEY, SECRET, CODE)
    }


    impl <'a> Blob<'a>{
        

        /// BY pinata client Definition only ipinata require to complete the task. There may be possible server loss connectivity. 
        ///
        /// # Examples
        ///
        /// ```
        /// use pinata_ipfs::ipinata::Blob;
        ///
        /// let mut blob = Blob{file : &PathBuf::from("abc.to_owned()"), api :"", token : "", status : FileStatus::Pin,  } ;
        /// assert_eq!(blob.pinta_client(), PinataApi{api_key : "", secret_api_key : ""});
        /// 
        /// ```
        pub fn pinta_client(&mut self) -> PinataApi {

           PinataApi::new(self.api, self.token).unwrap()
        }



        /// upload content definition require following parameters such as pinata ipfs client & file name. beause of async natue user will wait till process complete.
        pub async fn upload_content <'b>(&mut self, client : PinataApi, filename : String) -> Result<PinnedObject,ApiError> {

            let content = self.file.join(filename.to_owned()).display().to_string();
            self.status = FileStatus::Pin;
            
            client.pin_file(PinByFile::new(content)).await
        }


        /// delete content require hash of already uploaded content & pinata ipfs client
        pub fn delete_content<'b>(&mut self, client : PinataApi, hash : &'b str) -> std::io::Result<()>{

            let _ = client.unpin(hash);
            Ok(())
        }
        
    }


    /// change path allow you read a song from audio directory. [./Music]. Incase song might not be exist in music directory of a system irrelavant of os.
    /// There may be possible file return different path then throw error.
    ///
    /// # Examples
    ///
    /// ```
    /// use pinata_ipfs::ipinata::change_path;
    ///
    /// assert_eq!(change_path(dir, song), );
    /// ```
    pub fn change_path(dir : UserDirs, song : String) -> String{

        let mut relative_path : String = "".to_string();
        if let Some(path) = dir.audio_dir(){

            if !path.join(PathBuf::from(song.to_owned())).exists(){

                panic!("Error this file may be moved");
            }

            relative_path = path.display().to_string();
        }

        relative_path
    }

}


/// This ipfs client handle books on public network in decentralize way like music (pinata). The reason 
/// because pinata is very delicated network which is very good for specialize content while ipfs is open
/// which is good for other purposes.
/// 
/// 
pub mod ipfs_net{

    use directories::UserDirs;
    use pinata_sdk::{PinataApi, PinByFile};
    use std::{path::PathBuf, result::Result};
    use serde::{Deserialize, Serialize};

    use mongodb::{Client, Database, bson::doc, options::{ClientOptions, FindOptions}};
    use futures_util::{stream::TryStreamExt};

    #[derive(Debug)]
    pub struct IpfsBucket<'a>{

        name : String,
        file_ops : IpfsFileOp,
        format : &'a str,
        token : &'a str,

        #[warn(dead_code)]
        additional : IpfsFileAdvance,
    }

    #[derive(Debug)]
    pub enum IpfsFileOp{

        None,
        Upload,
        Download,
    }  

    #[derive(Debug)]
    pub enum IpfsFileAdvance{

        Stats,
        Pin,
        Unpin,
        None,
    }

    impl <'a> IpfsBucket<'a>{

        /// .
        ///
        /// # Examples
        ///
        /// ```
        /// use pinata_ipfs::ipfs_net::IpfsBucket;
        ///
        /// assert_eq!(IpfsBucket::new(name), IpfsBucket{name : "abc.to_owned().to_string()", file_ops : IpfsFileOp::None, format : ".pdf", additional : IpfsFileAdvance::None});
        /// ```
        pub fn new(name : String) -> Self{
            
            Self{

                name,
                file_ops : IpfsFileOp::None,
                format : ".pdf",
                additional : IpfsFileAdvance::None,
                token : "",
            }
        }

        /// Returns the get file path of this [`IpfsBucket`].
        ///
        /// # Examples
        ///
        /// ```
        /// use pinata_ipfs::ipfs_net::IpfsBucket;
        ///
        /// let mut ipfs_bucket = IpfsBucket::new(name);
        /// assert_eq!(ipfs_bucket.get_file_path(), "~/Download/abc.pdf");
        /// 
        /// ```
        pub fn get_file_path(&mut self) -> String {


            let mut cpath = "".to_string();

            if let Some(down_dir) = UserDirs::new() {
                
                if let Some(path) = down_dir.download_dir() {
                                        
                    if path.join(PathBuf::from(self.name.to_owned().to_string() + &self.format.to_owned().to_string())).exists(){

                        cpath = path.display().to_string();
                    }
                }
            }

            return cpath;
        }

        // create credentials is a private function which only accessible within module. This function return string literals as arguments.
    fn create_credentials<'b>(&mut self) -> (&'b str, &'b str, &'b str){

        const KEY : &'static str = "2a5fcc53ad9c7e814daa";
        const SECRET : &'static str = "496a37d27f698a32f7161d993455643f055e47606af98bc2ddf2c19403f4cb49";
        const  CODE : &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiIzOTA5YWE1NC1kNmI2LTQxN2MtYmQxZC1mMDVlZDE5ZDhmNzIiLCJlbWFpbCI6ImFsaWRldmVsb3Blcjk1QGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaW5fcG9saWN5Ijp7InJlZ2lvbnMiOlt7ImlkIjoiTllDMSIsImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxfV0sInZlcnNpb24iOjF9LCJtZmFfZW5hYmxlZCI6ZmFsc2UsInN0YXR1cyI6IkFDVElWRSJ9LCJhdXRoZW50aWNhdGlvblR5cGUiOiJzY29wZWRLZXkiLCJzY29wZWRLZXlLZXkiOiIyYTVmY2M1M2FkOWM3ZTgxNGRhYSIsInNjb3BlZEtleVNlY3JldCI6IjQ5NmEzN2QyN2Y2OThhMzJmNzE2MWQ5OTM0NTU2NDNmMDU1ZTQ3NjA2YWY5OGJjMmRkZjJjMTk0MDNmNGNiNDkiLCJpYXQiOjE2OTMzODgzMTZ9.bj-XI0u01IU8Gov0mjxwnh2lYdL9ln1rXvCNS5dwWrI";
        (KEY, SECRET, CODE)
    }

    pub fn pinta_client(&mut self) -> PinataApi {

            let (api, key, token) = self.create_credentials();
            self.token = token;
            PinataApi::new(api, key).unwrap()
    }


        pub async fn publish_book(&mut self, client : PinataApi, path : String) -> Result<pinata_sdk::PinnedObject, pinata_sdk::ApiError> {
            
            self.file_ops = IpfsFileOp::Upload;
            self.additional = IpfsFileAdvance::Pin;

            client.pin_file(PinByFile::new(PathBuf::from(path.to_owned()).join(self.name.to_owned().to_string() + & self.format.to_owned().to_string()).display().to_string())).await

        }

    }


    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Books{

        pub book : String,
        pub author : String,
        pub publisher : String,
        pub page : u16,
        pub description : String,
        pub ipfs_link : String,
        pub coonect : Peer,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Peer{

        pub session : String,
    }
    
    impl Books{

        pub fn new(book : String, author : String, publisher : String, page : u16, description : String, ipfs_link : String) -> Books{

            Self{
                book,
                author,
                publisher,
                page,
                description,
                ipfs_link,
                coonect: Peer{session : "".to_string()},
            }
        }

        pub async fn mongo_init() -> Client{

           Client::with_options(ClientOptions::parse("mongodb+srv://enigmabot:nigkjv8emfgPpoeI@streambusiness.nkakl0h.mongodb.net/").await.unwrap()).unwrap()

        }

        pub fn set_session(&mut self, session : String){

            self.coonect.session = session;
        }

        pub async fn get_session(&mut self) -> String {
            
            self.coonect.session.to_owned().to_string()
        
        }

        


        pub fn access_credentials(&mut self, client : Client) -> mongodb::Database {

            client.database("Artists_Record")
        }

        pub async fn create_book_doc(&mut self, db : Database) -> Result<String, String>{

          let col = db.collection::<Books>("enigmahouse");
          let book : Books;

          while let Ok(list) = db.list_collection_names(doc! {"name" : "enigmahouse"}).await{

             if list.is_empty(){
                
                book = Books{
                    book : self.book.to_owned(),
                    author : self.author.to_owned(),
                    publisher : self.publisher.to_owned(),
                    page : self.page.to_owned(),
                    ipfs_link : self.ipfs_link.to_owned(),
                    description : self.description.to_owned(),
                    coonect : self.coonect.to_owned(),
                };

                let _ = col.insert_one(book, None).await;
                break;
             }

             if list.len().ge(&1){
                
                let findbook = self.find_book_for_me(db.to_owned()).await.unwrap();
                
                if findbook.book.to_owned().to_string().eq(&self.book) && findbook.ipfs_link.to_owned().to_string().eq(&self.ipfs_link){
                    
                    return Err("This book already present in our database".to_string());
                
                } 
                
                book = Books{
                    book : self.book.to_owned(),
                    author : self.author.to_owned(),
                    publisher : self.publisher.to_owned(),
                    page : self.page.to_owned(),
                    ipfs_link : self.ipfs_link.to_owned(),
                    description : self.description.to_owned(),
                    coonect : self.coonect.to_owned(),
                };

                let _ = col.insert_one(book, None).await;
                break;
             }

          } 

          return Ok("".to_string());
                        
        }

        /// onself is a  method which return price of a rent space of a book for a user. 
        pub fn on_self(&mut self) -> f64{

            if self.page.to_owned().eq(&150){

                return 20.00;
            }
            
            if self.page.to_owned().ge(&200) && self.page.to_owned().le(&500){

                return 50.00;
            }

            if self.page.to_owned().ge(&500) && self.page.to_owned().eq(&700){

                return 100.00;
            }

            return 200.00;
        }

        pub async fn find_book_for_me(&mut self, db : Database) -> Result<Books, Books> {

            let collection = db.collection::<Books>("enigmahouse");
            let mut library : Books = Books { book: "".to_string(), author: "".to_string(), publisher: "".to_string(), page: 0, description: "".to_string(), ipfs_link: "".to_string(), coonect:  Peer { session: "".to_string() }};
    
            let findopts = doc!{"book" : self.book.to_owned()};
            let mut cursor = collection.find(findopts, None).await.unwrap(); 

            while let Ok(Some(record)) = cursor.try_next().await{
                

                if record.book.to_owned().eq(&""){
                    return Err(library);
                }else{

                    library = record;
                    break;
                }

                
                
            }
    
            Ok(library)
        }
        
    }
    
} 
