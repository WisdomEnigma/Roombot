# ImagetoVector 

        As the name suggested "ImagetoVector", which allow to operate on images. 

        Read images from Disk
        Convert images into vector notion.
        Register Face


# Install

        cargo add img2vec


# Documentation

        cargo doc --open


# Module 

        ImagetoVector
        Middleware

# Functions

        ImagetoVector 
                pub async fn open_image<'a>(path : String) -> DynamicImage // return image
                pub async fn new(dy_image : DynamicImage) -> ImagesVec // return class object
                
                                // convert image to vector
                pub async fn image_to_vec(&mut self, components : usize) -> Array2<f64> 

        vec_middleware

                pub async fn register_face() -> Result<(), std::io::Error>  // register face
                pub async fn create_index() -> Db  // create index in sled database 

                        // insert value in sled database
                pub async fn add_value(&mut self, mut authenicate : Authorization) -> std::io::Result<()>

# Maintainance 

        WISDOMENIGMA@2023
