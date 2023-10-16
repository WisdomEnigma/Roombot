/// Gatekeeper is a powerful library provide authenication of members. The algorithm is in blackbox.
/// Gatekeeper folllow rust modular apporached for more information read documentation or join our community.

/// This module published underthe licence of mozillia public licence, furthermore read licence terms. 

pub mod gatekeeper{


    // imports 
    use core::panic;
    use std::{hash::{Hash, SipHasher, Hasher}};
    use futures_util::{stream::TryStreamExt};
    use mongodb::{Client, options::{ClientOptions,FindOptions}, Database, bson::doc, results::InsertManyResult, };
    use serde::{Deserialize, Serialize};


    // static 

    // Endpoint is private static reference 
    static ENDPOINT : &str = "mongodb+srv://enigmabot:nigkjv8emfgPpoeI@streambusiness.nkakl0h.mongodb.net/";
    
    // DOC_NAME is another static refernce, both members perform task inside blackbox.
    static DOC_NAME : &str = "users";


    /// Profiler is a special object which has following fields {
    ///     email address & user-name
    /// }. This object traits derive from hash. Both fields are not acessible outside the module.
    #[derive(Hash)]
    pub struct Profiler{

        email : String,
        username : String,
    }


    /// new profile create instance a profiler . In simple words Agent allocation. Agent have some specific permissions which should be define in profiler implementation code.
    pub fn new_profile(email : String, username : String) -> Profiler{

        Profiler { email, username }
    }

    /// No implementation definition attached with profiler. 
    
    /// active hash is a generic Hash function which will return output of provided input.
    /// This output is machine generated and ambigious for humans. 
    pub fn active_hash<T : Hash>(t : &T) -> u64{

        let mut sip = SipHasher::new();
        t.hash(&mut sip);
        sip.finish()
    }


    // As the name reference verified is a private definition which verify the authenication inside the black box.  
    fn verified(old : String, new : String) -> bool{

        if old == new{
            return true;
        }
    
        false
    }


    /// mongodb is a public definition which return Result of mongodb instance. 
    pub async fn mongodb_client() -> Result<Client,mongodb::error::Error>{

        let client_opts = match ClientOptions::parse(ENDPOINT).await{
            Ok(options) => options,
            Err(e) => panic!("Error {:?}", e),
        };
        

        let client = Client::with_options(client_opts);
        client
    }

    /// Authenicate is another boss which have a permission to allocate agents on his behave. More information read Authenication definition.
    /// This instance derive from for traits 'debug', 'clone', 'serialize', 'deserialize'
    /// debug allow formatting
    /// deserialize json data to basic data
    /// serialize basic data to json data
    /// clone allow to copy reference.
    /// 
    /// Here username is only field accessible for public.
    #[derive(Debug,Deserialize, Serialize,Clone)]
    pub struct Authenicate{

        pub username : String,
        session : String,
        status : bool,

    }

    /// Authenication definition  
    /// 
    /// new  
    /// create_record 
    /// find with name
    /// find with session

    impl Authenicate{

        /// new definition return authenication instance. This function require key and value to operate in.
        pub fn new(values : String, k : String) -> Self{
            Self{
                username : k,
                session : values,
                status : true,  
            }
        }


        /// create record is very special function by definition .
        /// 
        /// create record operate under the influence of asynchornous which don't follow any pattern.
        /// create record require mutable self reference and db. Mutable reference self is not a function by a receiver.
        /// This function has only one task that is create record and return result.   
        pub async fn create_record(&mut self , db : Database) -> std::io::Result<()> {

           let collect = db.collection::<Authenicate>(DOC_NAME);
           
           let find_doc = self.find_with_session(self.session.to_string(), db.to_owned()).await;
           
           if !verified(find_doc.to_owned(), self.session.to_owned()){

                    let doc = vec![

                        Authenicate{

                            username : self.username.to_string(),
                            session : self.session.to_string(),
                            status : self.status,
                        },
                    ];

                    let _ = collect.insert_many(doc, None).await;
            }

           
           Ok(())
        }

        /// Again find with username is similar definition create record. This definition find paticular username for yourself. 
        pub async fn find_with_username(&mut self, value : String, database : Database) -> std::io::Result<()> {
            
             
            let collection = database.collection::<Authenicate>(DOC_NAME);
            
            let filter = doc!{ "username" : value.to_owned()};

            let find_opts = FindOptions::builder().sort(doc!{ "username" : 1}).build();
            let mut cursor = collection.find(filter, find_opts).await.unwrap();
            
            while let Some(profile) = cursor.try_next().await.unwrap(){

                if profile.username == " "{
                    panic!("Unforuente query must be empty ");
                }

                if profile.username != value{

                    panic!("No Data found ");
                } 
            }

            Ok(())
        }

        async fn find_with_session(&mut self, value : String, database : Database) -> String {
            

            let mut temp_session : String = "".to_string(); 
             
            let collection = database.collection::<Authenicate>(DOC_NAME);
            
            let filter = doc!{ "session" : value.to_owned()};

            let find_opts = FindOptions::builder().sort(doc!{ "session" : 1}).build();
            let mut cursor = collection.find(filter, find_opts).await.unwrap();
            
            while let Some(profile) = cursor.try_next().await.unwrap(){

                if profile.username == " "{
                    panic!("Unforuente query must be empty ");
                }

                if profile.session != value{

                    panic!("No Data found ");
                }

                temp_session = profile.session; 
            }

            temp_session
        }

    }
    
}
