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
    use imdb_async::{Client, Movie, Genre};
    use std::{time::Duration, path::Path};


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
    }
}