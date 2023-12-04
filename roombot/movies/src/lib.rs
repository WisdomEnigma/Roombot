///
/// Movies rating written under Mozillia Public Licence ; All the terms are clearly defined in MPL-2.0.
/// 
/// All the changes made according to wisdomenigma rules & MPL Licence terms. 
/// 
/// Redistribution, Commitment of work, Licence of Work, Intellectual Property & trademark.   
/// 
/// 
/// Contact us
///   github.com/WisdomEnigma                   wizdwarfs@gmail.com
/// 
/// 
/// 
/// 


pub mod movies_rating{


    use serde::{Deserialize, Serialize};
    use imdb_async::{Client, Movie, Genre, Show};
    use std::{time::Duration, path::Path, time::Instant};
    use once_cell::sync::OnceCell;

    pub static TV_SHOWS: OnceCell<String> = OnceCell::new(); // SEASON NAME
    pub static TV_SHOWS_RELEASE: OnceCell<u16> = OnceCell::new(); // SEASON RELEASE YEAR

    /// Movie Rate is a very powerful structure [name, release, genre, adult, watch_min,official].

    /// name => name of movie
    /// release => release of movie date,
    /// genre => EmotionFilter [action, adventure]
    /// imdb_id => imdb_id [1075611]
    /// adult => Movie type [Family or Adult]
    /// watch_min => Movie teaser watch
    /// official => imdb_link  
    /// 
    /// 
    /// MovieRate implement Debug trait

    #[derive(Debug)]
    pub struct MovieRate{

        pub name : String,
        pub release : u16,
        pub genre : Vec::<Emotionfilter>, 
        pub imdb_id : String,
        pub adult : Content,
        pub watch_min : u16,
        pub official : String,
        pub description : String,
    }

    /// Emotion Filter implement 4 traits Debug, Clone, Serialize & Deserialize
    /// 
    /// This filter type have more than 20 units structs. 
    /// These structs categorization movies. 
    /// 
    /// 
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Emotionfilter{

        Action,
        Adventure,
        Horror,
        Thriller,
        None,
        SciFi,
        Darma,
        Western,
        Comedy,
        Crime,
        Romance,
        Fantasy,
        Animation,
        Documentary,
        Music,
        Epic,
        History,
        Farce,
        Magic,
        Musical,
        Mystery,
        Expolit,
        Biography,
        Experimental,
        Family,
        FilmNoir,
        GameShow,
        News,
        RealityTv,
        Short,
        Sport,
        War,
        TalkShow,
        Adult,
    }


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ITV{

    title : Vec::<String>,
    season : Vec::<u16>,
    episode : Vec::<u16>,
    imdb_id : Vec::<u32>,
    year :  Vec::<std::option::Option<u16>>,
    minutes : Vec::<std::option::Option<u16>>,

}


    /// Content implement 4 traits like EmotionFilter
    /// 
    /// Content 2 unit struct [Family, Adult]
    /// 
    ///  
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Content{

        Adult,
        Family,
        None,
    }


    

    /// MovieRate provide following definitions 
    /// 
    /// 1. imdb client
    /// 2. imdb movie
    /// 3. imdb movies
    /// 4. movies iterator 

    impl MovieRate{


        /// imdb client is a asynchronus declaration which allow to connect imdb . Client definition require two 
        /// parameters cache time for data access & cache path. Because of asynchronous definition until 
        /// the process complete.
        pub async fn imdb_client() -> Client{

            Client::new(Duration::from_secs(30), Path::new("./movie")).await.unwrap()
        }

        /// new decalartion require 6 parameters. When user get access with MovieRate it should provide definition of these parameters. 
        pub fn new(name : String, release : u16, genre : Vec::<Emotionfilter>, imdb_id : String, adult : Content, watch_min : u16) -> Self{

            Self{
                name,
                release,
                genre,
                imdb_id,
                watch_min,
                adult,
                official : "".to_string(),
                description : "".to_string(),
            }
        }


        /// imdb movie is another async definition which search movie based on imdb id. 
        /// 
        /// 
        pub async fn imdb_movie(&mut self , mut client : Client) -> Movie{

            let id = self.imdb_id.to_string().parse::<u32>().unwrap();
            client.get_movie(id).await.unwrap()
        }


        /// imdb movies is similar like imdb movie except by this api search by name &  release of movie. Which is better than id.
        /// Because of lot of films release every weekend. This will return Option of Movie.
        /// 
        /// 
        pub async fn imdb_movies(&mut self, mut client : Client) -> Option<Movie>{

            client.get_movie_by_name_and_year(&self.name, self.release).await.unwrap()
        }

        pub async fn imdb_season(&mut self, mut client : Client) -> Option<Show>{
            
            client.get_show_by_name_and_year(&self.name, self.release).await.unwrap()
        }


        /// movie iterator is a normal declaration which will categorize movie based on class. 
        /// This process hardly take 200 msec. This process return further information about search movie. 
        /// This decalaration require imdb Movie object

        pub fn movies_iterator(&mut self, movie : Movie){

            let mut iterator = movie.genres().into_iter();
            
            
            for genre in iterator.by_ref(){

                match genre{

                    Genre::Action => {

                        self.genre.push(Emotionfilter::Action);
                    }

                    Genre::Adventure => {

                        self.genre.push(Emotionfilter::Adventure);
                    }

                    Genre::Animation => {

                        self.genre.push(Emotionfilter::Animation);
                    }
                    
                    Genre::Biography => {

                        self.genre.push(Emotionfilter::Biography);
                    }

                    Genre::Comedy => {

                        self.genre.push(Emotionfilter::Comedy);
                    }


                    Genre::Crime => {

                        self.genre.push(Emotionfilter::Crime);
                    }

                    Genre::Documentary => {

                        self.genre.push(Emotionfilter::Documentary);
                    }

                    Genre::Drama => {

                        self.genre.push(Emotionfilter::Darma);
                    }

                    Genre::Experimental => {

                        self.genre.push(Emotionfilter::Experimental);
                    }
                    
                    Genre::Family => {

                        self.genre.push(Emotionfilter::Family);
                    }

                    Genre::Fantasy => {

                        self.genre.push(Emotionfilter::Fantasy);
                    }


                    Genre::FilmNoir => {

                        self.genre.push(Emotionfilter::FilmNoir);
                    }

                    Genre::GameShow => {

                        self.genre.push(Emotionfilter::GameShow);
                    }

                    Genre::History => {

                        self.genre.push(Emotionfilter::History);
                    }

                    Genre::Horror => {

                        self.genre.push(Emotionfilter::Horror);
                    }
                    
                    Genre::Music => {

                        self.genre.push(Emotionfilter::Music);
                    }

                    Genre::Mystery => {

                        self.genre.push(Emotionfilter::Mystery);
                    }


                    Genre::Musical => {

                        self.genre.push(Emotionfilter::Musical);
                    }

                    Genre::News => {

                        self.genre.push(Emotionfilter::News);
                    }

                    Genre::RealityTv => {

                        self.genre.push(Emotionfilter::RealityTv);
                    }

                    Genre::Romance => {

                        self.genre.push(Emotionfilter::Romance);
                    }
                    
                    Genre::SciFi => {

                        self.genre.push(Emotionfilter::SciFi);
                    }

                    Genre::Short => {

                        self.genre.push(Emotionfilter::Short);
                    }


                    Genre::Sport => {

                        self.genre.push(Emotionfilter::Sport);
                    } 
                    
                    Genre::TalkShow => {

                        self.genre.push(Emotionfilter::TalkShow);
                    }
                    
                    Genre::Thriller => {

                        self.genre.push(Emotionfilter::Thriller);
                    }

                    Genre::War => {

                        self.genre.push(Emotionfilter::War);
                    }


                    Genre::Western => {

                        self.genre.push(Emotionfilter::Western);
                    }

                    Genre::Adult => {

                        self.genre.push(Emotionfilter::Adult);
                    }
                }
                 
            }

            if !movie.is_adult(){
            
                self.adult = Content::Family;
            
            }else{
    
                self.adult = Content::Adult;
            }
    
            if let Some(min) = movie.runtime_minutes(){

                self.watch_min =min;
            }

            self.official = "https://www.imdb.com/title/tt".to_string() + & self.imdb_id.to_owned().to_string();
            
            
            
        }

        pub async fn tv_shows(&mut self, itv : Show) {

            let dur = Instant::now();

            let mut iterator = itv.genres().into_iter();

            for genre in iterator.by_ref(){

                match genre{

                    Genre::Action=>{
                        self.genre.push(Emotionfilter::Action);
                    }

                    Genre::Adventure => {

                        self.genre.push(Emotionfilter::Adventure);
                    }

                Genre::Animation => {

                    self.genre.push(Emotionfilter::Animation);
                }
                
                Genre::Biography => {

                    self.genre.push(Emotionfilter::Biography);
                }

                Genre::Comedy => {

                    self.genre.push(Emotionfilter::Comedy);
                }


                Genre::Crime => {

                    self.genre.push(Emotionfilter::Crime);
                }

                Genre::Documentary => {

                    self.genre.push(Emotionfilter::Documentary);
                }

                Genre::Drama => {

                    self.genre.push(Emotionfilter::Darma);
                }

                Genre::Experimental => {

                    self.genre.push(Emotionfilter::Experimental);
                }
                
                Genre::Family => {

                    self.genre.push(Emotionfilter::Family);
                }

                Genre::Fantasy => {

                    self.genre.push(Emotionfilter::Fantasy);
                }


                Genre::FilmNoir => {

                    self.genre.push(Emotionfilter::FilmNoir);
                }

                Genre::GameShow => {

                    self.genre.push(Emotionfilter::GameShow);
                }

                Genre::History => {

                    self.genre.push(Emotionfilter::History);
                }

                Genre::Horror => {

                    self.genre.push(Emotionfilter::Horror);
                }
                
                Genre::Music => {

                    self.genre.push(Emotionfilter::Music);
                }

                Genre::Mystery => {

                    self.genre.push(Emotionfilter::Mystery);
                }


                Genre::Musical => {

                    self.genre.push(Emotionfilter::Musical);
                }

                Genre::News => {

                    self.genre.push(Emotionfilter::News);
                }

                Genre::RealityTv => {

                    self.genre.push(Emotionfilter::RealityTv);
                }

                Genre::Romance => {

                    self.genre.push(Emotionfilter::Romance);
                }
                
                Genre::SciFi => {

                    self.genre.push(Emotionfilter::SciFi);
                }

                Genre::Short => {

                    self.genre.push(Emotionfilter::Short);
                }


                Genre::Sport => {

                    self.genre.push(Emotionfilter::Sport);
                } 
                
                Genre::TalkShow => {

                    self.genre.push(Emotionfilter::TalkShow);
                }
                
                Genre::Thriller => {

                    self.genre.push(Emotionfilter::Thriller);
                }

                Genre::War => {

                    self.genre.push(Emotionfilter::War);
                }


                Genre::Western => {

                    self.genre.push(Emotionfilter::Western);
                }

                Genre::Adult => {

                    self.genre.push(Emotionfilter::Adult);
                }
            }
        }

        

            if let Some(min) = itv.runtime_minutes().to_owned(){
                
                self.watch_min = min;
            }
        
            if itv.is_adult(){

                self.adult = Content::Adult;
        
            }else{

                self.adult = Content::Family;
            }

            self.official = "https://www.imdb.com/title/tt".to_string() + & itv.imdb_id().to_owned().to_string();
            

            let elapsed = dur.elapsed();
            println!("Time duration of tv shows {:?}", elapsed);

            let _ = TV_SHOWS.set(self.name.to_owned().to_string());
            let _ = TV_SHOWS_RELEASE.set(self.release.to_owned());

                        
            
        }

        async fn get_episode_details(&mut self, mut client : Client) -> Vec::<ITV>{

            let mut it_v = Vec::<ITV>::new();

            let dur = Instant::now();

            if let Some(name) = TV_SHOWS.get(){

                if let Some(release) = TV_SHOWS_RELEASE.get(){

                    if name.eq(&self.name.to_owned()) && release.eq(&self.release.to_owned()){
                    
                        if let Some(itv) =  client.get_show_by_name_and_year(&self.name.to_owned(), self.release.to_owned()).await.unwrap(){

                            let mut episodes = Vec::<u16>::new();
                            let mut imdb_id = Vec::<u32>::new();
                            let mut title = Vec::<String>::new();
                             let mut season = Vec::<u16>::new();
                            let mut release = Vec::<std::option::Option<u16>>::new();
                            let mut minutes = Vec::<std::option::Option<u16>>::new();


                            let epic = itv.episodes().to_owned();
        
                            let tv = epic.into_iter();
                            for episode in tv {
      
                                    episodes.push(episode.to_owned().episode());
                                    imdb_id.push(episode.to_owned().imdb_id());
                                    season.push(episode.to_owned().season());
                                    release.push(episode.to_owned().year());
                                    minutes.push(episode.to_owned().runtime_minutes());
                                    title.push(episode.to_owned().title().to_string());
                            }
                        
                            it_v.push(ITV { title, season, episode: episodes, imdb_id, year: release, minutes });

                            self.name = itv.title().to_string();
                            self.release = itv.start_year();
                            self.imdb_id = itv.imdb_id().to_string();

                            let elapsed = dur.elapsed();
                            println!("Time duration of tv shows details {:?}", elapsed);

                        }
                    }   

                }
            }   

            
            it_v
        }      

        pub async fn get_episode(&mut self, client :Client) -> Vec::<ITV> {

           self.get_episode_details(client).await
           
        } 
        
        
        pub async fn get_episode_name(&mut self, itv : Vec::<ITV> ,  show :String) -> (i64, bool){

            let mut count: i64 = 0;
            let mut iterate = itv.into_iter();

            for data in iterate.by_ref(){

                let mut it = data.title.into_iter();

                for name in it.by_ref(){

                    if name.eq(&show){

                        return  (count, true);
                    }

                    count+=1;
                }
            }

            (-1, false) 
        }

        pub async fn get_episode_label(&mut self, itv : Vec::<ITV> ,  echo : i64, name : String) -> u16{

            
            let mut iterate = itv.into_iter();

            for data in iterate.by_ref(){

                if data.title[echo as usize].eq(&name){

                    return data.season[echo as usize];
                }
            }

            5000 
        }

        pub async fn get_episode_epic(&mut self, itv : Vec::<ITV> ,  echo : i64, name : String) -> u16{

            
            let mut iterate = itv.into_iter();

            for data in iterate.by_ref(){

                if data.title[echo as usize].eq(&name){

                    return data.episode[echo as usize];
                }
            }

            5000 
        }
    
        pub async fn get_episode_watch(&mut self, itv : Vec::<ITV> ,  echo : i64, name : String) -> std::option::Option<u16>{

            
            let mut iterate = itv.into_iter();

            for data in iterate.by_ref(){

                if data.title[echo as usize].eq(&name){

                    return data.minutes[echo as usize];
                }
            }

            Some(5000) 
        }

        pub async fn get_episode_id(&mut self, itv : Vec::<ITV> ,  echo : i64, name : String) -> u32{

            
            let mut iterate = itv.into_iter();

            for data in iterate.by_ref(){

                if data.title[echo as usize].eq(&name){

                    return data.imdb_id[echo as usize];
                }
            }

            5000 
        }
    
    }      

        
           
        
        
}