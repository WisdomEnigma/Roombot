pub mod Gatekeeper{

    use core::panic;
    use std::{hash::{Hash, SipHasher, Hasher}};
    use futures_util::{stream::TryStreamExt, future::ok};
    use mongodb::{Client, options::{ClientOptions,FindOptions}, Database,Collection, bson::Document, bson::doc, results::InsertManyResult, };
    use serde::{Deserialize, Serialize};

    static ENDPOINT : &str = "mongodb+srv://enigmabot:nigkjv8emfgPpoeI@streambusiness.nkakl0h.mongodb.net/";
    static DOC_NAME : &str = "users";

    #[derive(Hash)]

    pub struct Profiler{

        email : String,
        username : String,
    }


    pub fn new_profile(email : String, username : String) -> Profiler{

        Profiler { email, username }
    }

    pub fn active_hash<T : Hash>(t : &T) -> u64{

        let mut sip = SipHasher::new();
        t.hash(&mut sip);
        sip.finish()
    }

    fn verified(old : String, new : String) -> bool{

        if old == new{
            return true;
        }
    
        false
    }


    pub async fn mongodb_client() -> Result<Client,mongodb::error::Error>{

        let client_opts = match ClientOptions::parse(ENDPOINT).await{
            Ok(options) => options,
            Err(e) => panic!("Error {:?}", e),
        };
        

        let client = Client::with_options(client_opts);
        client
    }

    #[derive(Debug,Deserialize, Serialize,Clone)]
    pub struct Authenicate{

        pub username : String,
        session : String,
        status : bool,

    }

        

    impl Authenicate{

        pub fn new(values : String, k : String) -> Self{
            Self{
                username : k,
                session : values,
                status : true,  
            }
        }

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
