
/// Image2Vector Format transform any picture in vector .. There are following advanatages over 
/// Data already in compression mode . Data easily store in vector database. Mathematically data is encrypted but for mathematicans data easily extract.Less security enchancement techniques should applied.    

    

pub mod imagetovecformat{

    // import crates 
    use image::{DynamicImage, GenericImageView};
    use ndarray::{Axis, Array2,s};
    use ndarray_linalg::SVD;
    use std::path::{Path};
    
    /// open image allow to read image and return back. There may be possible image not exit or any other error then error reported
    pub async fn open_image<'a>(path : &Path) -> DynamicImage {

        // open image in a specific path (such as ~/Downloads) and extract value from Result
        let img : _ =  match image::open(path){

            Ok(image) => image,
            Err(err) => panic!("Error = {:?}", err),
        };

        img
    }

    /// ImageVec takes Dynamic Image as a field argument 
    #[derive(Debug)]
    pub struct ImagesVec{

        pub dy_image : DynamicImage
    }


    /// new function allow to create a ImageVec class object. 
    pub async fn new(dy_image : DynamicImage) -> ImagesVec{
        ImagesVec{dy_image : dy_image}
    }
    

    impl ImagesVec {


        /// This function takes image and tranform into mathematical notion called vectors. The Algorithm is simple..
        /// Convert picture into rgba pixels ; these pixels convert into pca value   
        pub async fn image_to_vec(&mut self, components : usize) -> Array2<f64> {

            // create and store rgba of a picture
            let platte : _ =  self.image_2_rgba_vec().await;
            
            // calculate pca of a picture after apply rgba
            Self::calculate_pca(&platte, components).await
        }

        /// This function calculate rgba value of an image and return Array2<f64>. 
        async fn image_2_rgba_vec(&mut self) -> Array2<f64>{

            // read image dimensions i.e width, height
            let (width, height) = (self.dy_image).dimensions();

            // read pixels values and store back in rgba array
            let mut rgba_platte = Array2::zeros((height as usize, width as usize * 4));
            for (x, y , pixels) in (self.dy_image).pixels(){

                let idx = x as usize * 4;
                rgba_platte[[y as usize ,idx]] = pixels[0] as f64;      // red
                rgba_platte[[y as usize , idx + 1]] = pixels[1] as f64; // green
                rgba_platte[[y as usize , idx + 2]] = pixels[2] as f64; // blue
                rgba_platte[[y as usize, idx + 3]] = pixels[3] as f64;  // alpha
            }

            rgba_platte

        }

        /// calculate_pca takes two parameters Array2<f64> reference & number of components as usize and return Array2<f64>
        async fn calculate_pca(data : &Array2<f64>, num : usize) -> Array2<f64> {

            // Mean of a data by providing Axis(0)
            let mean = data.mean_axis(Axis(0)).unwrap();

            // Center the data by subtracting the mean
            let centered = data - &mean;

            // Perform Singular Value Decomposition (SVD)
            let (_, _, v) = centered.svd(true, true).unwrap();

            // Extract the principal components
            let principal_components = v.expect("return pca ").slice(s![.., 0..num]).to_owned();

            principal_components

        }

        
    }

}



/// Vector Middleware ensure that every module work under proper directive. This module ensure authenication and optimize application workflow.
/// Vector Middle have connection with media storage  

pub mod vec_middleware{

    // import 
    use crate::{imagetovecformat};
    use vec_security::vec_security::{new_auth,Authorization};
    use ndarray::{Array2};
    use sled::{Db,IVec};
    use directories::{UserDirs};
    use std::path::{Path};
    
    /// Middleware require three fields (data = String, signature, verify = bool).
    #[derive(Debug)]
    pub struct Middleware{

        pub data : String,
        pub signature : String,
        pub verify : bool,

    }


    /// Register face is a public asyncronous function which will register your face in our database. 
    pub async fn register_face(db : Db) -> Result<(), std::io::Error> {
 
        // get directories from memory and create directory object. 
       if let Some(user_dir) = UserDirs::new(){

        // access download directory 
        
        if let Some(path) = user_dir.download_dir(){

            let p = path.clone();

            // if download don't have face then throw error.
            if !p.join(std::path::Path::new("register_face.png")).exists(){
                panic!("Error : {:?}", Errors::NotExit);
            }

            let joined = path.join(Path::new("register_face.png"));

            // read download directory and search for avatar.png
            let img = imagetovecformat::open_image(&joined);
       
            // store avatar image     
            let mut temp_img : _ = imagetovecformat::ImagesVec{
                dy_image : img.await,
            };

            // maximum pca components     
            let array : _ = temp_img.image_to_vec(100);
   
            // convert &Array[f64] into string    
            let face = set_data(array.await);

            // get face object as copy     
            let x : _ = face.clone();
   
            // use copy as argument     
            let authenicate : _ =  new_auth(x);

            // create Middleware object     
            let member = register_data(face, "".to_string(), false);

            // store user face in the database   
            let _ = member.await.add_value(authenicate.await, db);
        
            }
        
        }

        // return Result
        Ok(())
                            
    }

    

    /// unlock enure that record exit in our database and also unlock the account ; if user provide authenicate information 
    pub async fn unlock_account(db : Db) -> Option<IVec>{


        let mut v : Option<IVec> = Some(IVec::from(vec![0]));

        // get directories from memory and create directory object. 
       if let Some(user_dir) = UserDirs::new(){

        // access download directory 
        
        if let Some(path) = user_dir.download_dir(){

            
            let p = path;
            // if download don't have face then throw error.
            if !path.join(std::path::Path::new("avatar_unlock.png")).exists(){
                panic!("Error : {:?}", Errors::NotExit);
            }

            if !path.join(std::path::Path::new("register_face.png")).exists(){
                panic!("Error : {:?}", Errors::Unauthorized);
            }
        

            let joined = p.join(Path::new("avatar_unlock.png"));
            // read download directory and search for avatar.png
            let img : _ = imagetovecformat::open_image(&joined);
       
       
            // store avatar image     
      
            let mut temp_img : _ = imagetovecformat::ImagesVec{
           
                dy_image : img.await,
      
              };

       
              // maximum pca components     
      
              let array : _ = temp_img.image_to_vec(100);
      
       
              // convert &Array[f64] into string    
      
              let face = set_data(array.await);

      
              // create Middleware object     
     
              let member = register_data(face, "".to_string(), false);
     
              // retreive value by using key 
     
             let value = member.await.get_value(db).await;        
            
             v = value;
        }
    }

    v
    
}

    // create Middleware object
    async fn register_data(data : String, signature : String, verify : bool) -> Middleware{
        Middleware{
            data, 
            signature,
            verify,
        }
    }

    

    // set_data is a private function which will convert ndarray::Array2 into String 
    fn set_data(array : Array2<f64>) -> String {

        // apply slice on array, there may be possible that any error reported 
        // if not  then iterate , each value of array then map into string 
        // then array transform into vector of string and after that 
        // apply join function which transform vector of string into string
        array.as_slice().unwrap().iter().map(|x| format!("{:?}", x)).collect::<Vec::<String>>().join("")
    }

    // vector (array) unsigned 8-bit convert into String. 
    fn vecu8_to_string(data : Vec::<u8>) -> String{

        // apply slice on data then iterate over data vector, then map into string, collect all values in vector form and transform into string
        data.as_slice().iter().map(|x| format!("{:?}", x)).collect::<Vec::<String>>().join("")
    }

    


    /// create sled database object and return object. Here async is used in a function means, other tasks or process continue their work unless io, network
    /// async use await command which allow to pause the process till awaited process completed.    
    pub async fn create_index() -> Db{

        sled::open("user_Account").unwrap()
    }


    /// Errors_enums are easy way to report error
    #[derive(Debug)]
    pub enum Errors {
        
        Unauthorized,  //not allowed 
        Unverified,     // account still unverified ; sooner account will be disable,
        NotExit,        // data is not found 
        Duplicate,      // data replication
        None,           // all clear
    }


    //  Middleware provide sled functionalities insert, get, search and remove on data 
    impl Middleware{

        /// Middleware provide definition of add_value function which allow to store data in database.
        pub async fn add_value(&mut self, mut authenicate : Authorization, db : Db) -> std::io::Result<()> {
    
            // get hash of a datablock
            let new_hash : _ = authenicate.create_new_hash();
            
            // copy datablock into x.
            let x : _ = new_hash.clone();
            
            // verify datablock hash is valid 
            let verify : _  = authenicate.verified(new_hash);

            // retrive hash as string
            let encrypted : _ = vecu8_to_string(x);

            // if verify return false then return unauthorized error
            if !verify{

                panic!("Error : {:?}", Errors::Unauthorized);
            }

            // db insert operation have high priority ; user data added in database
            let _ = db.insert((self.data).as_bytes(), encrypted.as_bytes());

            // return ok result
            Ok(())
    
        }

        /// get is a sled operation which ensure data exist or throw valid exception if data not found
        pub async fn get_value(&mut self, client : Db) -> Option<IVec> {

            // allocate memory 
            let dataresult = &client.get(self.data.as_bytes()).unwrap().unwrap();

            // otherwise return Result::<IVec>
            Some(dataresult.clone())

        }

    }

    
    
}


