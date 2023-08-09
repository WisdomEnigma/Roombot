
/// Image2Vector Format transform any picture in vector .. There are following advanatages over 
/// Data already in compression mode . Data easily store in vector database. Mathematically data is encrypted but for mathematicans data easily extract.Less security enchancement techniques should applied.    


///   Tasks of this module 
/// 1. read image 
/// 2. transform into vector form
/// 3. save in vector database
/// 4. hashing 
/// 5. regenerated vector database back image
/// 6. face landmarks detect
/// 7. face landmarks remove 
/// 8. comparsion functionality    

pub mod imagetovecformat{

    // import crates 
    use image::{DynamicImage, GenericImageView};
    use ndarray::{Axis, Array2,s};
    use ndarray_linalg::SVD;


    
    /// read image from disk. This function return image,
    ///  if exist in a directory otherwise it report error
    pub async fn open_image<'a>(path : String) -> DynamicImage {

        // open image in a specific path (such as ~/Downloads) and extract value from Result
        // there may be possible [ok] return image or (error) throw error

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

    pub async fn new(dy_image : DynamicImage) -> ImagesVec{
        ImagesVec{dy_image : dy_image}
    }
    

    impl ImagesVec {


        /// This function takes image and tranform into mathematical notion called vectors. This function depend 
        /// on private functions "image_to_vec" & calculate_pca 
        pub async fn image_to_vec(&mut self, components : usize) -> Array2<f64> {

            // create and store rgba of a picture
            let platte : _ =  self.image_2_rgba_vec().await;
            
            // calculate pca of a picture after apply rgba
            Self::calculate_pca(&platte, components).await
        }

        /// This function calculate rgba value of an image and return Array2<f64>. 
        /// Array2 provide definition by ndarray crate.
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
    
    
    /// Middleware require three fields (data = String, signature, verify = bool).
    #[derive(Debug)]
    pub struct Middleware{

        pub data : String,
        pub signature : String,
        pub verify : bool,

    }


    /// Register face is a public asyncronous function which register your face in our database. 
    pub async fn register_face() -> Result<(), std::io::Error> {

        // read download directory and search for avatar.png
       let img : _ = imagetovecformat::open_image("~/Downloads/avatar.png".to_string());
       
        // store avatar image     
       let mut temp_img : _ = imagetovecformat::ImagesVec{
            dy_image : img.await,
       };

        // maximum pca components     
       let array : _ = temp_img.image_to_vec(100);
       
        // convert &Array[f64] into string    
       let conv : _ = array.await.as_slice().unwrap().iter().map(|x| format!("{:?}", x)).collect::<Vec::<String>>().join("");

        //    let member = new_member(conv, "".to_string(), false);
    
        
        // return Result
       Ok(())
    }

    /// this is a private function which create object called Middleware 
    async fn encrypt_face(data : String, signature : String, verify : bool) -> Member{
        Member{
            data,
            signature,
            verify,
        }
    }
}


