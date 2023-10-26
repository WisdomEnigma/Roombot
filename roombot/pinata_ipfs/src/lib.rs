pub mod ipinata{

    use std::{path::Path, path::PathBuf};
    use pinata_sdk::{PinataApi,PinByFile, PinnedObject, ApiError};
    use directories::UserDirs;
        
    pub struct Blob<'a>{

        file : &'a Path,
        api : &'a str,
        token : &'a str,
        status : FileStatus
    }


    pub enum FileStatus{
        Pin,
        Unpin,
    }


    pub fn new_bolb_object<'a>(file : &'a Path, operation : FileStatus) -> Blob<'a>{

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

        pub fn pinta_client(&mut self) -> PinataApi {

           PinataApi::new(self.api, self.token).unwrap()
        }

        pub async fn upload_content <'b>(&mut self, client : PinataApi, filename : String) -> Result<PinnedObject,ApiError> {

            let content = self.file.join(filename.to_owned()).display().to_string();
            self.status = FileStatus::Pin;
            client.pin_file(PinByFile::new(content)).await
        }

        pub fn delete_content<'b>(&mut self, client : PinataApi, hash : &'b str) -> std::io::Result<()>{

            let _ = client.unpin(hash);
            Ok(())
        }
        
    }

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
