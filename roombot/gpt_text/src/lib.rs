
/// This module provide wrapper plugins to connect with openai 

pub mod openai{

    // import fieri 

    use fieri::{Client, Error};
    use fieri::completion::{Completion, CompletionParamBuilder, create};
    use serde_json::{Value};
    use serde::{Deserialize};

    /// OpenAiCredentials (input = user query , end_user = "user_id", max_token = number of tokens) 
    #[derive(Debug, Deserialize)]
    pub struct OpenAICredentials{

        pub input : String,                          // meaning of my name !
        pub end_user : String,                       // abc78821-hkk789
        max_token : i32,                        // [0-500] 
    }

    /// openai module have own contructor which is accessible within module. This constructor require all the fields as arguments
    pub fn new(input : String, end_user : String, max_token : i32) -> OpenAICredentials{
        
        OpenAICredentials{
            input,
            end_user,
            max_token
        }
    }

    impl OpenAICredentials{

        /// openai_text_wrapper is a very basic function that solve user query linearly meaning , no additional parameters configuration
        /// similar like web search. Developer should have created openai key. Visit <http://www.openai.com>  
        pub async fn openai_text_wrapper(&mut self, apikey : String) -> Result<Completion, Error>{


            let client_fieri : _ = Client::new().api_key(apikey);
            let parameters : _ = CompletionParamBuilder::new("davinci")
            .prompt(self.input.clone())
            .temperature(0.5)
            .user(self.end_user.clone())
            .build()?;

            let response : _ = create(&client_fieri, &parameters);
            response.await
        }


        /// openai_openend is most complex function which solve query as equation. Here model reconfigure itself for finding optimal solution 
        /// composition of poetry or write blogs. Developer should have created openai key. Visit <http://www.openai.com>    
        pub async fn openai_openend(&mut self, apikey : String) -> Result<Completion, Error>{

            let client_fieri : _ = Client::new().api_key(apikey);
            let parameters : _ = CompletionParamBuilder::new("text-Davinci-003")
            .prompt(self.input.clone())
            .top_p(0.92)
            .max_tokens(self.max_token)
            .temperature(0.5)
            .frequency_penalty(0.5)
            .presence_penalty(0.5)
            .user(self.end_user.clone())
            .build()?;

            let response : _ =  create(&client_fieri, &parameters);
            response.await
        }



        ///openai_text_wrapper_as_json provide response in json object which is very beneficial for rest api calls. 
        /// This is a asynchronous function and compactible with asynchronous caller functions.  
        pub async fn openai_text_wrapper_as_json(&mut self, apikey : String) -> Value{

            let mut datacall : _ = new(self.input.clone(),self.end_user.clone(), self.max_token);
            let json_obj : _ = match datacall.openai_text_wrapper(apikey).await{

                Ok(resp) => resp,
                Err(err) => panic!("Error = {:?}", err),
            };

            serde_json::to_value(&json_obj).unwrap()
        }

        

    }

    
}

