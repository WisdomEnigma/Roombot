
/// Image2Vector Format transform any picture in vector .. There are following advanatages over 
/// Data already in compression mode . Data easily store in vector database. Mathematically data is encrypted but for mathematicans data easily extract.Less security enchancement techniques should applied.    


///   Tasks of this module 
/// 1. read image 
/// 2. transform into vector form
/// 3. save in vector database
/// 4. cryptographic ecllipted curve 
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

        let img : _ =  match image::open(path){

            Ok(image) => image,
            Err(err) => panic!("Error = {:?}", err),
        };

        img
    }

    /// ImageVec takes Dynamic Image as a field argument 
    #[derive(Debug)]
    pub struct ImagesVec{

        dy_image : DynamicImage
    }

    pub async fn new(dy_image : DynamicImage) -> ImagesVec{
        ImagesVec{dy_image : dy_image}
    }
    

    impl ImagesVec{


        /// This function takes image and tranform into mathematical notion called vectors. This function depend 
        /// on private functions "image_to_vec" & calculate_pca 
        pub async fn image_to_vec(&mut self, components : usize) {

            let platte : _ =  self.image_2_rgba_vec().await;
            
            Self::calculate_pca(&platte, components).await;
        }


        /// This function calculate rgba value of an image and return Array2<f64>. 
        /// Array2 provide definition by ndarray crate.
        async fn image_2_rgba_vec(&mut self) -> Array2<f64>{

            let (width, height) = (self.dy_image).dimensions();

            let mut rgba_platte = Array2::zeros((height as usize, width as usize * 4));
            for (x, y , pixels) in (self.dy_image).pixels(){

                let idx = x as usize * 4;
                rgba_platte[[y as usize ,idx]] = pixels[0] as f64;
                rgba_platte[[y as usize , idx + 1]] = pixels[1] as f64;
                rgba_platte[[y as usize , idx + 2]] = pixels[2] as f64;
                rgba_platte[[y as usize, idx + 3]] = pixels[3] as f64;
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




