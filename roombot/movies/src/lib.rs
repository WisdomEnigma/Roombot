pub mod movies_rating{


    use serde::{Deserialize, Serialize};
    use imdb_async::{Client, Movie, Genre};
    use std::{time::Duration, path::Path};


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

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Content{

        Adult,
        Family,
        None,
    }


    impl MovieRate{

        pub async fn imdb_client() -> Client{

            Client::new(Duration::from_secs(30), Path::new("./song")).await.unwrap()
        }


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

        pub async fn imdb_movie(&mut self , mut client : Client) -> Movie{

            let id = self.imdb_id.to_string().parse::<u32>().unwrap();
            client.get_movie(id).await.unwrap()
        }

        pub async fn imdb_movies(&mut self, mut client : Client) -> Option<Movie>{

            client.get_movie_by_name_and_year(&self.name, self.release).await.unwrap()
        }

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