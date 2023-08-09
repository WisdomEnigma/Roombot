
/// Vector_security ensure that user data remain secure. 
/// These module build a unique signature against users.
pub mod vec_security {

    // import 
    use crypto_hash::{Algorithm, Hasher};
    use std::io::Write;
    use regex::{Regex};
    

    #[derive(Debug)]
    /// Authorization is a class where only one field exit, that is data. That data transforme into mathematical notion called hashing  
    pub struct Authorization{

        data : String,

    }

    /// new_auth is a public method which takes data as argument and return authorize object.
    pub async fn new_auth(data : String) -> Authorization{
        Authorization{
            data,
        }
    }

    impl Authorization{


        /// Create new hash which is act as receiver and return vector.  
        pub fn create_new_hash(&mut self) -> Vec::<u8>{

            let mut hash_data : _ = Hasher::new(Algorithm::SHA256);
            let _ = hash_data.write_all((self.data).as_bytes());

            hash_data.finish()
        }


        /// verified is a special function that return signature is authorize or not 
        pub fn verified(&mut self, data : Vec::<u8>) -> bool{

            

            let word : _ = String::from_utf8(data).unwrap();
            let pattern : _ = Regex::new("#[a-z]([0-9])").unwrap();
            
           if pattern.is_match(&word) {

                return true;
           }

           false

        }
    }
}


