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
    use std::path::PathBuf;
    use ipfs_api::{IpfsClient, Form, IpfsApi};
    use std::io::Cursor;

    #[derive(Debug)]
    pub struct IpfsBucket<'a>{

        name : String,
        file_ops : IpfsFileOp,
        format : &'a str,
        additional : IpfsFileAdvance,
    }

    #[derive(Debug)]
    pub enum IpfsFileOp{

        Copy,
        Mv,
        Remove,
        None,
        Upload,
        Download,
    }  

    #[derive(Debug)]
    pub enum IpfsFileAdvance{

        Stats,
        Size,
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

            if let Some(down_dir) = UserDirs::new() {
                if let Some(path) = down_dir.download_dir() {
                    if !path.join(PathBuf::from(self.name.to_owned())).exists(){

                        return "".to_string();
                    }

                    return path.display().to_string();
                }
            }

            return "".to_string();
        }

        fn ipfs_client(&mut self) -> IpfsClient{

            IpfsClient::default()
        }


        /// ipfs_file add allow you to save file on ipfs public network.  The advantage over this network it's secure & purely decentralized.
        /// How ? Because it's public network and depend on content routing algorithm which is similar like url of content.
        pub async fn ipfs_file_add(&mut self, path : String) -> Result<Vec<ipfs_api::response::AddResponse>, ipfs_api::Error> {

            let mut form = Form::default();

            self.file_ops = IpfsFileOp::Upload;

            let file = PathBuf::from(self.name.to_owned().to_string()).join(self.format.to_owned()).display().to_string();
            form.add_reader_file(path, Cursor::new(Vec::new()), file);

            let ipfs_client = self.ipfs_client();
            let _dns = ipfs_client.dns("ipfs.io", true).await;
            

            let add = ipfs_api::request::Add{
                wrap_with_directory : Some(true),
                ..Default::default()
            };
            
            ipfs_client.add_with_form(form, add).await
        }



    }
} 
