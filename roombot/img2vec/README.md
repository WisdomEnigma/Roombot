# ImagetoVector 

        As the name suggested "ImagetoVector", which allow to operate on images. 

        Read images from Disk
        Convert images into vector notion.
        Register Face


# Install

        cargo add img2vec



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

                        // insert value in sled database
                pub async fn add_value(&mut self, mut authenicate : Authorization, db : Db) -> std::io::Result<()> 

                        // get value from sled database
                pub async fn get_value(&mut self, client : Db) -> std::io::Result<IVec>

        pub async fn register_face() -> Result<(), std::io::Error>  // register face
        pub async fn create_index() -> Db  // create index in sled database 
        pub async fn unlock_account(db : Db) -> std::io::Result<()> // unlock account


# native-dependenices 

        Img2vec is only compactible with linux distos because linux features are linked. Soon this module avaible for other operating systems as well. 

        OS              Supported               Upcoming
        Linux               Yes                 Optimized
        Mac                 No                  Optimized + native-lib
        Windows             No                  Optimized + native-lib

# Maintainance 

        WISDOMENIGMA@2023
