
/// This module provide wrapper plugins to connect with openai 

pub mod openai{

    // import fieri 

    use fieri::{Client, Error};
    use fieri::completion::{Completion, CompletionParamBuilder, create};


    pub struct OpenAICredentials{

        input : String,                          // meaning of my name !
        end_user : String,                       // abc78821-hkk789
        max_token : i32,                        // [0-500] 
    }

    pub fn new(input : String, end_user : String, max_token : i32) -> OpenAICredentials{
        
        OpenAICredentials{
            input,
            end_user,
            max_token
        }
    }

    impl OpenAICredentials{

        /// openai_text_wrapper is a very basic function that solve user query linearly meaning , no additional parameters configuration
        /// similar like web search 
        pub async fn openai_text_wrapper(&mut self) -> Result<Completion, Error>{


            let client_fieri : _ = Client::new();
            let parameters : _ = CompletionParamBuilder::new("davinci")
            .prompt(self.input.clone())
            .temperature(0.5)
            .user(self.end_user.clone())
            .build()?;

            let response : _ = create(&client_fieri, &parameters);
            response.await
        }


        /// openai_openend is most complex function which solve query as equation. Here model reconfigure itself for finding optimal solution 
        /// composition of poetry or write blogs   
        pub async fn openai_openend(&mut self) -> Result<Completion, Error>{

            let client_fieri : _ = Client::new();
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

    }

    
}
