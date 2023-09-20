pub mod lightnode_net{


    use nodeless_rs::{Nodeless, ServerStatusResponse};
    
    static APIKEY : &str = "164|J7568txDMheEJTQPGseeu615xChXu4caSSceX89m3ade7ea8";

    static STORE : &str = "e1be7458-9364-4f40-8de0-22a3d5af8db5";

    #[derive(Debug)]
    pub struct INodeless<'a>{

        pub amount : f64,
        pub email :  &'a str,
        pub status : TransactionStatus,
        pub remaining : f64,
        duration : i64,   
    } 

    #[derive(Debug)]
    pub enum TransactionStatus {

        Paid,
        PartialPaid,
        Expire,
        Process,
    }

    impl <'a> INodeless<'a>{

        pub fn new(amount : f64, email : &'a str, remain : f64, due : i64) -> Self{

            Self{

                amount,
                email,
                remaining: remain,
                duration : due,
                status : TransactionStatus::Process,
            }
        }

        pub async fn connect_with_store(&mut self) -> ServerStatusResponse{

         let lbtc =  Nodeless::new(APIKEY, None).unwrap();
          lbtc.get_server_status().await.unwrap()
        }
    }

}
