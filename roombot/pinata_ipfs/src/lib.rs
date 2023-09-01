

/// IPINTATA is a wrapper module allow to upload and download content from Pinata IPFS. 
/// IPINATA contain Distributed Content object which is futhermore depend on Gateway.
pub mod ipinata{

    use std::path::Path;
    use reqwest::multipart::{Part,Form};
    use reqwest::Client;
    use std::fs::File;
    use std::io::Read;
    use reqwest::StatusCode;
    use reqwest::header::HeaderMap;

    /// Pinata url used for upload & download assets from pinata

    const PINATA_URL : &'static str = "https://api.pinata.cloud/pinning/pinFileToIPFS";

    /// Distibuted Content have following requirements 
    /// File : which file you want to upload ; where it locate (relative path)
    /// PriceperWork : how much cost of your work.
    /// cidversion : [0, 1]
    /// Delicated Network :  [Special , ROUTINE]
    pub struct DistributedContent<'a>{

        pub file : &'a Path,
        pub priceperwork : f64,
        pub cidversion : i32,
        pub delicated_network : Gateway,
        pub file_name : &'a str,

        apikey : &'a str,
        secret : &'a str,
        jwt : &'a str,  
    }


    /// Gateway ensure Routine for all people while Special for secure and special network
    pub enum Gateway{
        SPECIAL_PURPOSE,
        ROUTLINE_DATA,
    }

    /// new a public function which create distributed content. new function require string literals. 
    pub fn new<'a>(file : &'a Path, price : f64, network : Gateway, file_name : &'a str) -> DistributedContent<'a>{

        let (primary, partial, forgein) = create_credentials();

        DistributedContent{
            file : file,
            priceperwork : price,
            cidversion : 0,
            delicated_network : network,
            file_name,

            apikey : &primary,
            secret : &partial,
            jwt : &forgein,
        }
    }


    // create credentials is a private function which only accessible within module. This function return string literals as arguments.
    fn create_credentials<'a>() -> (&'a str, &'a str, &'a str){

        const KEY : &'static str = "2a5fcc53ad9c7e814daa";
        const SECRET : &'static str = "496a37d27f698a32f7161d993455643f055e47606af98bc2ddf2c19403f4cb49";
        const  CODE : &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiIzOTA5YWE1NC1kNmI2LTQxN2MtYmQxZC1mMDVlZDE5ZDhmNzIiLCJlbWFpbCI6ImFsaWRldmVsb3Blcjk1QGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaW5fcG9saWN5Ijp7InJlZ2lvbnMiOlt7ImlkIjoiTllDMSIsImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxfV0sInZlcnNpb24iOjF9LCJtZmFfZW5hYmxlZCI6ZmFsc2UsInN0YXR1cyI6IkFDVElWRSJ9LCJhdXRoZW50aWNhdGlvblR5cGUiOiJzY29wZWRLZXkiLCJzY29wZWRLZXlLZXkiOiIyYTVmY2M1M2FkOWM3ZTgxNGRhYSIsInNjb3BlZEtleVNlY3JldCI6IjQ5NmEzN2QyN2Y2OThhMzJmNzE2MWQ5OTM0NTU2NDNmMDU1ZTQ3NjA2YWY5OGJjMmRkZjJjMTk0MDNmNGNiNDkiLCJpYXQiOjE2OTMzODgzMTZ9.bj-XI0u01IU8Gov0mjxwnh2lYdL9ln1rXvCNS5dwWrI";
        (KEY, SECRET, CODE)
    }


    /// create payload return request payload. However this function is public 
    pub fn create_payload<'a>() -> &'a str{

        const PAYLOAD : &'static str = "-----011000010111000001101001\r\nContent-Disposition: form-data; name=\"cidVersion\"\r\n\r\n0\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name=\"wrapWithDirectory\"\r\n\r\nfalse\r\n-----011000010111000001101001--\r\n\r\n";

        PAYLOAD
    }


    /// Distributed Content provide definition of upload and download on pinata data.    
    impl <'a> DistributedContent<'a>{


        /// By definition upload is an asynchronous function and require distributed content object. Initally file must be 
    /// exist , otherwise throw error. Read image file and create new form that will submitted back to 
    /// pinata. Payload attached with request. Check whether response return success , otherwise throw back error.
        pub async fn upload_images(&mut self) -> Result<StatusCode, Box<dyn std::error::Error>> {

            let mut file = match File::open(self.file){

                Ok(file) => file,
                Err(e) => panic!("Error {:?}", e),
            };

            let mut header_map = HeaderMap::new();
            header_map.insert("accept", "application/json".parse().unwrap());
            header_map.insert("content-type", "multipart/form-data; boundary=---011000010111000001101001".parse().unwrap());

            let mut image_data = Vec::new();

            let _ = file.read_to_end(&mut image_data);
            let client = Client::new();
            let cid = "{".to_owned() +"cidversion : " + "0}";

            let form = Form::new()
                                .text("pinata_api_key", self.apikey.to_string())
                                .text("pinata_secret_key", self.secret.to_string())
                                .text("pinataOptions", cid)
                                .part("image", Part::bytes(image_data));

            let response = client.post(PINATA_URL).headers(header_map).multipart(form).send().await;

            let response_body = match response{
                Ok(response) => response,
                Err(e) => panic!("{:?}", e), 
            };
            
            if !response_body.status().is_success(){
                panic!("Error : process fail while upload data {:?}", StatusCode::NOT_FOUND);
            }

            Ok(StatusCode::OK)
        }

    }

}
