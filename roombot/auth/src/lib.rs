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
        let mut terminal = term::stdout().unwrap();

        t.hash(&mut sip);


        terminal.fg(term::color::GREEN).unwrap();

        write!(terminal, "[login] Authorize your identity .. \n").unwrap();
        
        sip.finish()
    }


    // As the name reference verified is a private definition which verify the authenication inside the black box.  
    fn verified(old : String, new : String) -> bool{

        let mut terminal = term::stdout().unwrap();

        if old.to_owned().eq(&new){

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "[verify] Welcome buddy ... \n ").unwrap();
            return true;
        }

        terminal.fg(term::color::BLUE).unwrap();

        write!(terminal, "[new user ] Welcome to our community .... \n  ").unwrap();
    
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


    /// login expire validate whether session is active or not 
    pub fn login_expire(me : u64) -> bool {
        
        let mut terminal = term::stdout().unwrap();

        if me.to_owned().eq(&0) {
                

                terminal.fg(term::color::RED).unwrap();

                write!(terminal, "[session] your session expire , re login ... \n  ").unwrap();            
                return true;
        }

        terminal.fg(term::color::GREEN).unwrap();

        write!(terminal, "[session] plenty of time ... \n  ").unwrap();
        
        false
        
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
        newsletter : bool,
        pub email : String,
        refer_email : String,
        refer_name : String,
        invitation : bool,
    }

    /// Authenication definition  
    /// 
    /// new  
    /// create_record 
    /// find with name
    /// find with session
    /// 


    impl Authenicate{

        /// new definition return authenication instance. This function require key and value to operate in.
        pub fn new(session : String, username : String, email : String) -> Self{
            Self{
                username,
                session,
                newsletter : false,
                email,
                refer_email : "".to_string(),
                refer_name : "".to_string(),
                invitation : false,  
            }
        }


        /// create record is very special function by definition .
        /// 
        /// create record operate under the influence of asynchornous which don't follow any pattern.
        /// create record require mutable self reference and db. Mutable reference self is not a function by a receiver.
        /// This function has only one task that is create record and return result.   
        pub async fn create_record(&mut self , db : Database) -> std::io::Result<()> {

           let collect = db.collection::<Authenicate>(DOC_NAME);

           let mut terminal = term::stdout().unwrap();

            terminal.fg(term::color::GREEN).unwrap();

            write!(terminal, "Looking for information.... \n  ").unwrap();
           
           let find_doc = self.find_with_session(self.session.to_string(), db.to_owned()).await.unwrap();
           
           if !verified(find_doc.session.to_owned(), self.session.to_owned()) &&  find_doc.email.to_owned().ne(&self.email){

                    let doc = vec![

                        Authenicate{

                            username : self.username.to_owned().to_string(),
                            session : self.session.to_owned().to_string(),
                            newsletter : false,
                            email : self.email.to_owned(),
                            refer_email : self.refer_email.to_owned(),
                            refer_name : self.refer_name.to_owned(),
                            invitation : self.invitation.to_owned(),
                        },
                    ];

                    terminal.fg(term::color::GREEN).unwrap();

                    write!(terminal, "[cred] SIGNUP PROCESS COMPLETE ... \n  ").unwrap();

                    let _ = collect.insert_many(doc, None).await;
            }

           
           Ok(())
        }

        /// Again find with username is similar definition create record. This definition find paticular username for yourself. 
        pub async fn find_with_username(&mut self, value : String, database : Database) -> std::io::Result<()> {
            
             
            let collection = database.collection::<Authenicate>(DOC_NAME);

            let mut terminal = term::stdout().unwrap();

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "Looking for information ...  \n  ").unwrap();
            
            let filter = doc!{ "username" : value.to_owned()};

            let find_opts = FindOptions::builder().sort(doc!{ "username" : 1}).build();
            let mut cursor = collection.find(filter, find_opts).await.unwrap();
            
            while let Some(profile) = cursor.try_next().await.unwrap(){

                if profile.username.to_owned().eq(&" "){

                    terminal.fg(term::color::BRIGHT_RED).unwrap();

                    write!(terminal, "[result] by chance ! we found empty bit .... \n  ").unwrap();

                    panic!("Unforuente query must be empty ");
                }

                if profile.username != value{

                    terminal.fg(term::color::RED).unwrap();

                    write!(terminal, "[result] MISSING PIECE NOT CONNECTED ... \n ").unwrap();

                    panic!("No Data found ");
                } 
            }

            Ok(())
        }

        /// When user login his or her account. Session will be created... 
        async fn find_with_session(&mut self, value : String, database : Database) -> Result<Authenicate,Authenicate> {
            

            let mut credentials = Authenicate{
                username : "".to_string(), 
                session : "".to_string(), 
                email : "".to_string(), 
                newsletter : false, 
                refer_email : "".to_string(),
                refer_name : "".to_string(),
                invitation : false
            };

             
            let collection = database.collection::<Authenicate>(DOC_NAME);

            let mut terminal = term::stdout().unwrap();

            terminal.fg(term::color::GREEN).unwrap();

            write!(terminal, "Looking for information ... \n ").unwrap();
            
            let filter = doc!{ "session" : value.to_owned()};

            let find_opts = FindOptions::builder().sort(doc!{ "session" : 1}).build();
            let mut cursor = collection.find(filter, find_opts).await.unwrap();
            
            while let Some(profile) = cursor.try_next().await.unwrap(){

                if profile.username.to_owned().eq(&" "){

                    terminal.fg(term::color::BRIGHT_RED).unwrap();

                    write!(terminal, "[empty] ohh! we caught empty bit :(... \n  ").unwrap();
                    panic!("Unforuente query must be empty ");
                }

                if profile.session != value{


                    terminal.fg(term::color::RED).unwrap();

                    write!(terminal, "[result] missing piece not found ... \n  ").unwrap();
                    
                    panic!("No Data found ");
                }

                credentials = profile;
                break;
            }

            terminal.fg(term::color::GREEN).unwrap();

            write!(terminal, "[result] well done , this is the result of your task... \n ").unwrap();

            Ok(credentials) 
        }

    }
    
}

/// Accounts handle user related tasks such as add user information ; update information; find person from record.
pub mod accounts{


    use futures_util::{future::ok, TryStreamExt };
    use mongodb::{Client, options::{ClientOptions, FindOptions, CountOptions, FindOneAndUpdateOptions}, Database, bson, bson::doc};
    use serde::{Deserialize,Serialize}; 
    
    #[derive(Debug, Serialize, Deserialize, Clone)]

    /// INFO hold information about user's (first name, lastname, work, company, education etc). Some fields are public & rest are not.. public fields are directly accessible for more information read documenation 
    pub struct Info{

        pub firstname : String,
        pub lastname : String,
        pub institute : Vec::<String>,
        pub degree : Vec::<String>,
        pub workplace : Vec::<String>,
        pub city : String,
        pub country : String,
        pub bitcoinaddr : Vec::<String>,
        pub work : String,
        pub session : String,
        pub address : Vec::<String>,
        pub fblink : String,
        pub instalink : String,
        pub xlink : String,
        pub youlink : String,
        pub new_digital : Vec<String>,
        pub network : Vec::<ProfileNetwork>,
        pub cfollowers : i64,
        pub cfavouite : i64,
        pub open : bool,
        taste : Taste,
        pub hobby : Vec::<Hobby>,
        pub phonenum : String,
        personality : Vec::<String>, 
        pub collection_books : Vec<FavBooks>, 
        pub collection_movies : Vec<FavMovies>,
        pub collection_brand : Vec<FavBrands>,
        pub collection_place : Vec<FavPlace>,
        pub bio : String,
        pub weburl : String,
        pub discord_url : String,
        pub threads_url : String,
        pub mastodon_url : String,
    }

    /// User might choose one or many hobbies ...

    /// golf game
    pub const GOLF  : &'static str = "Golf";
    
    /// cricket 
    pub const CRICKET : &'static str = "Cricket";
    
    /// tennis 
    pub const TENNIS : &'static str = "Tennis";
    
    /// car racing 
    pub const CARRACING : &'static str = "Car_Racing";
    
    /// swim 
    pub const SWIMMING  : &'static str = "Swim";
    
    /// bike racing
    pub const BIKERACING : &'static str = "Bike_Racing";
    
    /// hunt
    pub const HUNTING  : &'static str = "Hunting";
    
    /// travel
    pub const TRAVEL : &'static str = "Travel";
      
    /// chess
    pub const CHESS : &'static str = "Chess";
    
    /// monopoly
    pub const MONOPOLY : &'static str = "Monopoly";
    
    /// cook
    pub const COOKING : &'static str = "Cooking";
    
    /// welfare
    pub const SOCAILWELFARE : &'static str = "SocialWelfare";

    /// profile network help you to maintain network . followers & favourite are open fields 
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ProfileNetwork{

        pub followers : bool,
        pub favourite : bool,
    }


    /// Favourite Books store data about your favourite books  

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FavBooks{

        books : String,
        recomemded : bool,
        authors : String,
        watch : bool,
    }

    /// Favourite Movies store data about your favourite movies 
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FavMovies{

        movies : String,
        recomemded : bool,
        artist : String,
        watch : bool,
    }

    /// Favourite Place store data about your favourite place 
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct  FavPlace{

        lastvisited : String,
        recomemded : bool,
        plantotrip : String,
        watch : bool,
    }


    /// Favourite Brand store data about your favourite brand 

    #[derive(Debug, Serialize, Deserialize, Clone)]

    pub struct FavBrands{

        recommeded : bool,
        favourite_b : String,
        watch : bool,
    }

    /// Some people are highly networth choose Affluent & rest are public
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Taste {

        Affluent,
        Public,
        None,
    }

    /// Some user's have unique taste such golf, cricket etc
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Hobby {
        
        Game(String),
        None,
    }


    #[derive(Debug, Serialize, Deserialize, Clone)]
    /// Interest are [technology, science, fashion & beauty, law, real estate, banking & finance ] etc
    pub enum Interest{

        Interest(String),
        None,
    }

    /// followers trait should implement , because it's an provide open ended questions.  

    pub trait Followers{
        
        fn follower(&mut self, index : usize);  

        fn unfollow(&mut self, index : usize);

        fn total_followers(&mut self);

        fn reduce_followers(&mut self);
    }

    /// likwise followers , favourite is also trait & you should implement before interact with info.
    pub trait Favourite{

        fn connection(&mut self, index : usize);
        
        fn undo(&mut self, index : usize);

        fn total(&mut self);

        fn reduce(&mut self);

    }

    /// open network allow user to control his or her information while build network. 

    pub trait OpenNetwork {
        
        fn open (&mut self);

        fn private(&mut self);

    }


    /// character is your reptuation either you are worth or reckless 
    pub trait Chacter{

        fn worth(&mut self, atributes : Vec::<String>);
        fn reckless(&mut self, atributes : Vec::<String>);
        fn endorse(&mut self, attribute : String) -> bool;
    }


    impl Chacter for Info{

        fn endorse(&mut self, atribute : String) -> bool{
            
            if self.personality.is_empty() {return false;}
            
            for entity in self.personality.to_owned(){

                if entity.contains(&atribute){ return true;}
            }

            return false;
        }

        fn reckless(&mut self, atributes : Vec::<String>) {
            
            for mut entity in self.personality.to_owned(){

                for a in atributes.to_owned(){

                    if entity.contains(&a){

                        entity.pop();
                        continue;
                    }        
                    
                }
            }
        }

        fn worth(&mut self, atributes : Vec::<String>) {
            
            self.personality = atributes;

        }
    }


    /// followers have following operations ( follower, unfollower, total_followers, reduce_followers)
    /// folllow make friend 
    /// unfollow remove friend 
    /// total_followers how many friend i have
    /// reduce_followers selected group  

    impl Followers for Info {
        
        fn follower(&mut self, index : usize){

            let mut network : ProfileNetwork = ProfileNetwork{followers : false, favourite : false}; 

            if self.network.len().ge(&0) && index.le(&self.network.len()) && index.ge(&0) {

                network.followers = true;

                self.network.push(network);
            }
            
            
        }

        fn unfollow(&mut self, index : usize){

            let mut network : ProfileNetwork = ProfileNetwork{followers : false, favourite : false}; 

            if self.network.len().ge(&0) && index.ge(&0) {

                network.followers = false;

                self.network.push(network);
            }
        }

        fn total_followers(&mut self){

            self.cfollowers+= 1;
        }


        fn reduce_followers(&mut self) {
            
            self.cfollowers -=1;
        }
                
    }

    impl Favourite for Info{

        fn connection(&mut self, index : usize) {
            
            let mut network : ProfileNetwork = ProfileNetwork{followers : false, favourite : false}; 

            if self.network.len().ge(&0) && index.le(&self.network.len()) && index.ge(&0) {

                network.favourite = true;

                self.network.push(network);
            }
        }

        fn reduce(&mut self) {
            
            self.cfavouite -=1;
        }

        fn total(&mut self) {
            
            self.cfavouite +=1;
        }

        fn undo(&mut self, index : usize) {
            
            let mut network : ProfileNetwork = ProfileNetwork{followers : false, favourite : false}; 

            if self.network.len().ge(&0) && index.ge(&0) {

                network.favourite = false;

                self.network.push(network);
            }
        }
    }


    /// open network provide following definition (open means social connection network . in short friend of friend is my friend)..
    /// while private 180 degree opposite meaning (only few people are might be our friend).  
    impl OpenNetwork for Info{

        fn open (&mut self) {
            
            self.open = true;
        }

        fn private(&mut self) {
            
            self.open = false;
        }
    }

    impl Info{

        /// create instance of user info
        ///
        /// # Examples
        ///
        /// ```
        /// use auth::accounts::Info;
        ///
        /// let result = Info::new(firstname, lastname, institute, degree, workplace, work, city, country, bitcoinaddr);
        /// assert_eq!(result, Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()});
        /// ```
        pub fn new(firstname : String, lastname : String, 
        institute : String, degree : String, 
        workplace : String, work : String,
        city : String, country : String, bitcoinaddr : String, phonenum : String) -> Info{
            
            let mut academica = Vec::<String>::new();
            academica.push(institute);

            let mut achieve = Vec::<>::new();
            achieve.push(degree);

            let mut  place = Vec::<>::new();
            place.push(workplace);

            let mut home_address = Vec::<>::new();
            home_address.push("".to_string());

            let mut btc_wallet = Vec::<>::new();
            btc_wallet.push(bitcoinaddr);


            Self { 
                
                firstname, 
                lastname, 
                institute : academica, 
                degree : achieve, 
                workplace : place, 
                city, 
                country, 
                work,
                bitcoinaddr : btc_wallet, 
                session : "".to_string(),
                address : home_address,
                fblink : "".to_string(),
                instalink : "".to_string(),
                xlink : "".to_string(),
                youlink : "".to_string(),
                new_digital : Vec::<String>::new(),
                network : Vec::<ProfileNetwork>::new(),
                cfavouite : 0,
                cfollowers : 0,
                open : false,
                taste: Taste::None,
                hobby : Vec::<Hobby>::new(),
                phonenum : phonenum,
                personality : Vec::<String>::new(),
                collection_books : Vec::<FavBooks>::new(),
                collection_brand : Vec::<FavBrands>::new(),
                collection_movies : Vec::<FavMovies>::new(),
                collection_place : Vec::<FavPlace>::new(),
                bio : "".to_string(),
                weburl : "".to_string(),
                discord_url : "".to_string(),
                mastodon_url : "".to_string(),
                threads_url : "".to_string(),
             }

        }

        /// mongo_init allow to create mongoclient instance for record keeping purpose. 
        pub async fn mongo_init(&mut self) -> Client{

            Client::with_options(ClientOptions::parse("mongodb+srv://enigmabot:nigkjv8emfgPpoeI@streambusiness.nkakl0h.mongodb.net/").await.unwrap()).unwrap()
 
         }
 
        /// set user session  
         pub fn set_session(&mut self, session : String){
 
             self.session = session;
         }
 
        /// get user session  
         pub async fn get_session(&mut self) -> String {
             
             self.session.to_owned().to_string()
         
         }

        ///  get credentials 
         pub fn access_credentials(&mut self, client : Client) -> Database {

            client.database("Artists_Record")
        }

        /// create record doc bookkeep user information, if information should not be existed before in our record. 
         ///
        /// # Examples
        ///
        /// ```
        ///     use auth::accounts::Info;
        ///
        ///     let mut info = Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()}); 
        ///     unsafe{
        /// 
        ///         my_info.set_session("1568..".to_owned().to_string()); 
        ///     }
        /// 
        ///     let mongo = my_info.mongo_init().await;
        ///     let cred = my_info.access_credentials(mongo);
        ///     let record = match my_info.create_record_doc(cred).await
        /// 
        ///     assert_eq!(info.create_record_doc(cred), Ok("".to_string());
        /// ```
        pub async fn create_record_doc(&mut self, db : Database) -> Result<String, String>{

                let col = db.collection::<Info>("accounts");

                let mut terminal = term::stdout().unwrap();

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "Looking for information ... \n ").unwrap();

                let data = self.getaccount(db.to_owned()).await.unwrap();

                if data.firstname.to_owned().eq(&"") && data.lastname.to_owned().eq(&"") && data.bitcoinaddr.len().eq(&0){

                    let info = Info{ 
                        firstname: self.firstname.to_owned(), 
                        lastname: self.lastname.to_owned(),
                        institute: self.institute.to_owned(),
                        degree: self.degree.to_owned(),
                        workplace: self.workplace.to_owned(),
                        city: self.city.to_owned(),
                        country: self.country.to_owned(),
                        bitcoinaddr: self.bitcoinaddr.to_owned(),
                        work: self.work.to_owned(), 
                        session: self.session.to_owned(),
                        address: self.address.to_owned(), 
                        fblink: self.fblink.to_owned(), 
                        instalink: self.instalink.to_owned(), 
                        xlink: self.xlink.to_owned(), 
                        youlink: self.youlink.to_owned(), 
                        new_digital: self.new_digital.to_owned(),
                        network : self.network.to_owned(),
                        cfavouite : self.cfavouite,
                        cfollowers : self.cfollowers, 
                        open : self.open.to_owned(),
                        taste : self.taste.to_owned(),
                        hobby : self.hobby.to_owned(),
                        phonenum : self.phonenum.to_owned(),
                        personality : self.personality.to_owned(),
                        collection_books : self.collection_books.to_owned(),
                        collection_brand : self.collection_brand.to_owned(),
                        collection_movies : self.collection_movies.to_owned(),
                        collection_place : self.collection_place.to_owned(),
                        bio : self.bio.to_owned(),
                        discord_url : self.discord_url.to_owned(),
                        mastodon_url : self.mastodon_url.to_owned(),
                        threads_url : self.threads_url.to_owned(),
                        weburl : self.weburl.to_owned()
                    };

                    let _ = col.insert_one(info, None).await;
                    
                    terminal.fg(term::color::GREEN).unwrap();
                
                    write!(terminal, "[create ] Successfully Create operation complete .... \n   ").unwrap();
                }

                return Ok("".to_string()) 
         }


        /// count people is a special method which return how many user's exist in our bookkeeping record. E.g "Ali" => 5.   
        ///
        /// # Examples
        ///
        /// ```
        /// use auth::accounts::Info;
        ///
        /// let mut info = Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()}); 
        ///     unsafe{
        /// 
        ///         my_info.set_session("1568..".to_owned().to_string()); 
        ///     }
        /// 
        ///     let mongo = my_info.mongo_init().await;
        ///     let cred = my_info.access_credentials(mongo);
        ///     assert_eq!(info.count_people(cred).await, Ok(5));
        /// ```
        pub async fn count_people(&mut self, db : Database) -> Result<u64, u64>{

                let mut counter : u64 = 0;

                let col = db.collection::<Info>("accounts");

                let mut terminal = term::stdout().unwrap();

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "Looking for information ... \n ").unwrap();

                while let Ok(record) = col.count_documents(doc! {"firstname" : doc! {"$exists" : true}}, None).await{

                        if record.to_owned().eq(&0){

                            terminal.fg(term::color::RED).unwrap();

                            write!(terminal, "[result] Human's are so much confused , barr ... missing piece is not located .... \n  ").unwrap();
                            return Err(record)
                        }


                        if record.to_owned().ge(&1){

                            counter = record;
                            break;
                        }

                }

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "[result] Oh yeah missing piece found ... \n  ").unwrap();

                Ok(counter)  
         }

        /// find people with name as name specify , which allow extract name from our record  
        ///
        /// # Examples
        ///
        /// ```
        /// use auth::accounts::Info;
        ///
        ///     let mut info = Info{"abc".to_string(), "xyz".to_string(), 
        ///             "".to_string(), "".to_string(),"".to_string(),"".to_string(), 
        ///             "".to_string(), "".to_string(), "b......................1j".tostring()}); 
        ///     unsafe{
        /// 
        ///         my_info.set_session("1568..".to_owned().to_string()); 
        ///     }
        /// 
        ///     let mongo = my_info.mongo_init().await;
        ///     let cred = my_info.access_credentials(mongo);
        ///     
        ///     assert_eq!(info.find_people_with_name(cred), Ok([Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()}]));
        /// ```
        pub async fn find_people_with_name(&mut self, db : Database) -> Result<Vec<Info>, Vec<Info>>{

            
           let mut info : Vec::<Info> = Vec::<_>::new();

           let col = db.collection::<Info>("accounts");

           let mut terminal = term::stdout().unwrap();

            terminal.fg(term::color::GREEN).unwrap();

            write!(terminal, "Looking for information ... \n ").unwrap();

            let mut find_doc = col.find(doc! {"firstname" : self.firstname.to_owned()}, FindOptions::builder().sort(doc!{ "firstname" : 1}).build()).await.unwrap();

            while let Ok(Some(record)) = find_doc.try_next().await  {

                if record.firstname.to_owned().eq(&"") { 

                    terminal.fg(term::color::RED).unwrap();
    
                    write!(terminal, "[empty] Human's information is so much scatter connect dot by dot like move mountain ... \n  ").unwrap(); 
                    return Err(info);
                }
                
                if record.firstname.to_owned().eq(&self.firstname){ info.push(record.to_owned());}

                if record.firstname.to_owned().ne(&self.firstname){ continue; }

                break;
            }

            terminal.fg(term::color::GREEN).unwrap();

            write!(terminal, "[result] oh yeah ! missing piece found ... \n  ").unwrap();
            
            Ok(info)
         }

        /// transaction status is a powerful function which authorize user & user have access to secure digital wallet for transaction purpose
        /// 
        /// ```
        ///     use auth::accounts::Info;
        ///
        ///     let mut info = Info{"abc".to_string(), "xyz".to_string(), 
        ///             "".to_string(), "".to_string(),"".to_string(),"".to_string(), 
        ///             "".to_string(), "".to_string(), "b......................1j".tostring()}); 
        ///     unsafe{
        /// 
        ///         my_info.set_session("1568..".to_owned().to_string()); 
        ///     }
        /// 
        ///     let mongo = my_info.mongo_init().await;
        ///     let cred = my_info.access_credentials(mongo);
        ///     
        ///     assert_eq!(info.transaction_status(cred), Ok("b...........1j".to_string()));
        /// ```
         pub async fn transaction_status(&mut self, db : Database) -> Result<String, String>{

            let mut myaddress = "".to_string(); 
            
            let col = db.collection::<Info>("accounts");

            let mut terminal = term::stdout().unwrap();

            terminal.fg(term::color::GREEN).unwrap();

            write!(terminal, "Looking for information ... \n ").unwrap();

            let mut iterate = col.find(doc!{"session" : self.session.to_owned()}, None).await.unwrap();
 
            while let Ok(Some(record)) = iterate.try_next().await{

                if record.session.to_owned().to_string().is_empty(){

                    terminal.fg(term::color::RED).unwrap();

                    write!(terminal, "[empty] You humans ! are so much complex ... \n  ").unwrap();

                    return Err("No record".to_string());
                }

                if record.bitcoinaddr[0].to_owned().to_string().is_empty(){


                    terminal.fg(term::color::RED).unwrap();

                    write!(terminal, "[result] ahh! so much confused ... \n ").unwrap();

                    return Err("No bitcoin address provided".to_string());
                }

                myaddress = record.bitcoinaddr[0].to_owned();
                break;
            }

            terminal.fg(term::color::GREEN).unwrap();

            write!(terminal, "[result] finally ! dots connect .... \n  ").unwrap();

            return Ok(myaddress)
         }

         /// this feature allow us to retrieve information about user, if user information should be exist  
        ///
        /// # Examples
        ///
        /// ```
        /// use auth::accounts::Info;
        ///
        ///     let mut info = Info{"abc".to_string(), "xyz".to_string(), 
        ///             "".to_string(), "".to_string(),"".to_string(),"".to_string(), 
        ///             "".to_string(), "".to_string(), "b......................1j".tostring()}); 
        ///     unsafe{
        /// 
        ///         my_info.set_session("1568..".to_owned().to_string()); 
        ///     }
        /// 
        ///     let mongo = my_info.mongo_init().await;
        ///     let cred = my_info.access_credentials(mongo);
        ///     
        ///     assert_eq!(info.get_account(cred), Ok([Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()}]));
        /// ```

        pub async fn getaccount(&mut self, db : Database) -> Result<Info, Info>{

            let mut info = Info{ firstname: "".to_string(), 
            lastname: "".to_string(), 
            institute: Vec::<>::new(), 
            degree: Vec::<>::new(), 
            workplace: Vec::<>::new(), 
            city: "".to_string(), 
            country: "".to_string(), 
            bitcoinaddr: Vec::<>::new(), 
            work: "".to_string(), 
            session: "".to_string(), 
            address: Vec::<>::new(), 
            fblink: "".to_string(), 
            instalink: "".to_string(),
            xlink: "".to_string(), 
            youlink: "".to_string(), 
            new_digital: Vec::<String>::new(), 
            network : self.network.to_owned(),
            cfavouite : self.cfavouite,
            cfollowers : self.cfollowers, 
            open : self.open.to_owned(),
            taste : self.taste.to_owned(),
            hobby : self.hobby.to_owned(),
            phonenum : self.phonenum.to_owned(),
            personality : self.personality.to_owned(),
            collection_books : self.collection_books.to_owned(),
                        collection_brand : self.collection_brand.to_owned(),
                        collection_movies : self.collection_movies.to_owned(),
                        collection_place : self.collection_place.to_owned(),
                        bio : self.bio.to_owned(),
                        discord_url : self.discord_url.to_owned(),
                        mastodon_url : self.mastodon_url.to_owned(),
                        threads_url : self.threads_url.to_owned(),
                        weburl : self.weburl.to_owned()
         };

            let collection = db.collection::<Info>("accounts");

            let mut terminal = term::stdout().unwrap();

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "Looking for information ...  \n ").unwrap();

            let opts = FindOptions::builder().sort(doc!{ "firstname" : 1}).build();

            let mut query = collection.find(doc!{"firstname" : self.firstname.to_owned()}, opts).await.unwrap();

            while let Ok(Some(account_info_col)) = query.try_next().await{

                info = account_info_col.to_owned();
                break;
            }

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "[result] finally ! dots are connected ... ").unwrap();

            Ok(info)
        }


        /// get account with user session return user information if user session is known , this function require admistrative right.
        /// 
        /// # Examples
        ///
        /// ```
        ///     use auth::accounts::Info;
        ///
        ///     let mut info = Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()}); 
        ///     unsafe{
        /// 
        ///         my_info.set_session("1568..".to_owned().to_string()); 
        ///     }
        /// 
        ///     let mongo = my_info.mongo_init().await;
        ///     let cred = my_info.access_credentials(mongo);
        ///     assert_eq!(info.get_account_with_user_session(cred).await, Ok(Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()...}));
        /// ```

        async fn get_account_with_user_session(&mut self, db : Database) -> Result<Info, Info>{

            let mut info = Info{ 
                    firstname: "".to_string(), 
                    lastname: "".to_string(), 
                    institute: Vec::<>::new(), 
                    degree: Vec::<>::new(), 
                    workplace: Vec::<>::new(), 
                    city: "".to_string(), 
                    country: "".to_string(), 
                    bitcoinaddr: Vec::<>::new(), 
                    work: "".to_string(), 
                    session: "".to_string(), 
                    address: Vec::<>::new(), 
                    fblink: "".to_string(), 
                    instalink: "".to_string(),
                    xlink: "".to_string(), 
                    youlink: "".to_string(), 
                    new_digital: Vec::<String>::new(), 
                    network : self.network.to_owned(),
                    cfavouite : self.cfavouite,
                    cfollowers : self.cfollowers, 
                    open : self.open.to_owned(),
                    taste : self.taste.to_owned(),
                    hobby : self.hobby.to_owned(),
                    phonenum : self.phonenum.to_owned(),
                    personality : self.personality.to_owned(),
                    collection_books : self.collection_books.to_owned(),
                        collection_brand : self.collection_brand.to_owned(),
                        collection_movies : self.collection_movies.to_owned(),
                        collection_place : self.collection_place.to_owned(),
                        bio : self.bio.to_owned(),
                        discord_url : self.discord_url.to_owned(),
                        mastodon_url : self.mastodon_url.to_owned(),
                        threads_url : self.threads_url.to_owned(),
                        weburl : self.weburl.to_owned()
            };

            let collection = db.collection::<Info>("accounts");

            let mut terminal = term::stdout().unwrap();

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "Looking for information \n ").unwrap();

            let opts = FindOptions::builder().sort(doc!{ "session" : 1}).build();

            let mut query = collection.find(doc!{"session" : self.session.to_owned()}, opts).await.unwrap();


                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "[process] missing piece ! ... \n  ").unwrap();

            while let Ok(Some(account_info_col)) = query.try_next().await{

                info = account_info_col.to_owned();
                break;
            }

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "[result] finally ! uphill task complete .... \n  ").unwrap();

            Ok(info)
        }

        /// set taste definition of enum taste which two options provided [ affluent, public]
        pub fn set_taste(&mut self, myclass : Taste){


            let mut terminal = term::stdout().unwrap();

            match myclass {
                
                Taste::Affluent =>  {

                    terminal.fg(term::color::BRIGHT_GREEN).unwrap();

                    write!(terminal, "[result] AFFLUENT, I RESPECT THAT .... \t ").unwrap();

                    self.taste = myclass;
                },
                Taste::Public => {
                    

                    terminal.fg(term::color::GREEN).unwrap();

                    write!(terminal, "[result] Ordinay ... \t ").unwrap();
                    
                    self.taste = myclass
                },
                Taste::None => {

                    terminal.fg(term::color::BRIGHT_RED).unwrap();

                    write!(terminal, "[result] No taste .... \t  ").unwrap();
                    
                    eprintln!(" taste should not be provided :");
                }
            }
        }

        pub fn taste(&mut self) -> Taste{

            return self.taste.clone();
        }


        /// set user personality 
        pub fn set_personality(&mut self, character : String){

            self.personality.push(character);
        }

        pub fn personality(&mut self) -> Vec::<String>{

            return self.personality.clone();
        }


        /// update follower method update databse whenever user connection update meaning either he/she make new friend or reject him/her.
        /// # Examples
        ///
        /// ```
        ///     use auth::accounts::Info;
        ///
        ///     let mut info = Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()}); 
        ///     unsafe{
        /// 
        ///         my_info.set_session("1568..".to_owned().to_string()); 
        ///     }
        /// 
        ///     let mongo = my_info.mongo_init().await;
        ///     let cred = my_info.access_credentials(mongo);
        ///     assert_eq!(info.update_follower(cred).await, Ok());
        /// ``` 
        pub async fn update_follower(&mut self, db : Database) -> Result<(), ()>{

            let collect = db.collection::<Info>("accounts");

            let mut terminal = term::stdout().unwrap();

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "Looking for information ... \n ").unwrap();

            let update_doc = doc! {
                "$set" : {
                    "cfollowers" : self.cfollowers,
                    "network" : bson::to_bson(&self.network).unwrap(),
                },
            };

            let update_opts = FindOneAndUpdateOptions::builder().return_document(mongodb::options::ReturnDocument::After).build();
            while let Some(data) =  collect.find_one_and_update(doc!{ "firstname" : self.firstname.to_owned()}, update_doc.to_owned(), update_opts.to_owned()).await.unwrap(){

                if data.firstname.to_owned().eq(&""){

                    terminal.fg(term::color::RED).unwrap();

                    write!(terminal, "[result] ohh! empty string ...  \n ").unwrap();
                    
                    eprintln!("sorry error occurred while updating data ");
                    return Err(());
                }

                if data.firstname.to_owned().eq(&self.firstname){


                    terminal.fg(term::color::GREEN).unwrap();

                    write!(terminal, "[result] finally ! update your data ... \n ").unwrap();
                    println!("congrats operation complete successfuuly");
                    break;
                }
            }

            Ok(())

        }


        /// update close method update databse whenever user build close relationship such as family or friend's
        /// # Examples
        ///
        /// ```
        ///     use auth::accounts::Info;
        ///
        ///     let mut info = Info{"abc".to_string(), "xyz".to_string(), "".to_string(), "".to_string(),"".to_string(),"".to_string(), "".to_string(), "".to_string(), "b......................1j".tostring()}); 
        ///     unsafe{
        /// 
        ///         my_info.set_session("1568..".to_owned().to_string()); 
        ///     }
        /// 
        ///     let mongo = my_info.mongo_init().await;
        ///     let cred = my_info.access_credentials(mongo);
        ///     assert_eq!(info.update_close(cred).await, Ok());
        ///
        /// ``` 

        pub async fn update_close(&mut self, db : Database) -> Result<(), ()>{

            let collect = db.collection::<Info>("accounts");

            let mut terminal = term::stdout().unwrap();

                terminal.fg(term::color::GREEN).unwrap();

                write!(terminal, "Looking for information .... \n ").unwrap();

            let update_doc = doc! {
                "$set" : {
                    "cfavouite" : self.cfavouite,
                    "network" : bson::to_bson(&self.network).unwrap(),
                },
            };

            let update_opts = FindOneAndUpdateOptions::builder().return_document(mongodb::options::ReturnDocument::After).build();
            while let Some(data) =  collect.find_one_and_update(doc!{ "firstname" : self.firstname.to_owned()}, update_doc.to_owned(), update_opts.to_owned()).await.unwrap(){

                if data.firstname.to_owned().eq(&""){

                    terminal.fg(term::color::RED).unwrap();

                    write!(terminal, "[result] ohh! we found empty bit ... \n  ").unwrap();
                    
                    eprintln!("sorry error occurred while updating data ");
                    return Err(());
                }

                if data.firstname.to_owned().eq(&self.firstname){

                    terminal.fg(term::color::GREEN).unwrap();

                    write!(terminal, "[result] finally data update .... \n  ").unwrap();

                    println!("congrats operation complete successfuuly");
                    break;
                }
            }

            Ok(())

        }
    }
}