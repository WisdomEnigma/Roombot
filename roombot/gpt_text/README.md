# gpt_text_wrapper


	This is a encaplusate version of openai version called "fieri" . The basic function of this module is to reduce the excutation cycles such as nlp related tasks.


 		
# structure 	
	
      gpt_text 
	  |------------- src
			|-------------lib.rs
	  |------------- cargo.toml


# module 

	-> openai

# code 

	use gpt_text::openai;

	let mut openc : _ = openai::new("cat translate into japanese".to_string(), "".to_string(), "cat".len().try_into().unwrap());
        
	let wrap_gpt : _ = openc.openai_text_wrapper(openai_apikey).await{

            Ok(text) => text,
            Err(err) => panic!("Error = {:?}", err),
    };

	println!("Response = {:?}", wrap_gpt);


# install

	cargo add gpt_text

# documentation 

	cargo doc --open	 