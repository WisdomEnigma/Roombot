

/// lightnode_net is a solution of old economy problem which is centralization. Centralization prior their interest over others. i.e bank acquire more resources for expansion then society betterment.
/// So we use a solution called lightnode, which is designed by best of best minds. "bitcoin"
///  This module is accessible for public.

pub mod lightnode_net{

    // imports
    use mongodb::{Database,bson, bson::doc, options::{FindOneAndUpdateOptions, FindOptions}};
    use serde::{Serialize, Deserialize};
    use url::Url;
    use std::str::FromStr;
    use futures_util::stream::TryStreamExt;
    use nodeless_rs::{Nodeless, store::InvoiceRequest, transaction::Transaction};
    
    
    
    
    // static private members
    static APIKEY : &str = "164|J7568txDMheEJTQPGseeu615xChXu4caSSceX89m3ade7ea8";
    static STORE : &str = "e1be7458-9364-4f40-8de0-22a3d5af8db5";
    static LEDGER_BIT : &str = "ledgertxs";

    /// INodeless have seven public fields [amount, email, status, remain, name , session and light_id].
    /// This instance have four derive traits. [clone, debug, serialize & deserialize]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct INodeless{

        pub amount : u64,
        pub email :  String,
        pub status : TransactionStatus,
        pub remaining : f64,
        pub name :String,
        pub session : String,
        pub lid : String,

    } 

    /// transaction status is enumerate [abstract type]. By defintion custom type enforce instance traits.
    #[derive(Debug,Clone, Serialize, Deserialize)]
    pub enum TransactionStatus {

        Process,
        Expire,
        Deposit,
        Pending,
    }


    /// INodeless definition 
    /// ***********************
    /// new
    /// create_nodeless_client
    /// connect_with_store
    /// lightnode_store_inovice
    /// store_inovice
    /// store_status
    /// get_store_tnx
    /// 
    /// ************************
    /// from_txs
    /// update_tnx
    /// get_tnx_record
    impl INodeless{

        // new instance
        pub fn new(amount : u64, email : String, remain : f64, name : String, id : String, process : TransactionStatus, ld : String) -> Self{

            Self{

                amount,
                email,
                remaining: remain,
                status : process,
                name,
                session : id,
                lid : ld,
            }
        }

        /// By definition it will access to lightnode_net instance. Lightnode have many benefits 
        /// they are not centralized , they are fast , secure and last wireless transaction. 
        pub async fn create_nodeless_client(&mut self) -> Nodeless{

         Nodeless::new(APIKEY, None).unwrap()
          
        }

        /// connect with store definition is to access store for the service exchange.
        pub async fn connect_with_store(&mut self, node : &Nodeless) -> Result<nodeless_rs::store::Store, nodeless_rs::error::NodelessError>{

            node.get_store(STORE).await
        }


        /// lightnode store invoice generte inovice when user will ready for exchange hands
        pub async fn lightnode_store_inovice(&mut self, node : &Nodeless) -> Result<nodeless_rs::store::Invoice, nodeless_rs::error::NodelessError> {


           let inovice = InvoiceRequest{
            
            amount : self.amount as f64,
            currency : "USD".to_string(),
            buyer_email : self.email.to_string(),
            redirect_url : Url::from_str("https://nodeless.io").unwrap(),
            metadata : None,
           };
            
            node.create_store_invoice(STORE, inovice).await
        }

        /// from txs allow to record every information for furture purpose.
        pub async fn from_txs (&mut self, db : Database) -> std::io::Result<()>{

            let collect = db.collection::<INodeless>(LEDGER_BIT);

            let data = self.get_tnx_record(db).await;

            if self.session != data.session{

                let doc = vec![INodeless{

                amount : self.amount,
                email : self.email.to_owned(),
                remaining: self.remaining,
                status : self.status.clone(),
                name : self.name.to_owned(),
                session :self.session.to_owned(),
                lid : self.lid.to_owned(),
            },
        ];

           let _ =  collect.insert_many(doc, None).await;

        }
            
            Ok(())
        }



        /// update tnx update transactional information ; if require
        pub async fn update_tnx (&mut self, db : Database) -> std::option::Option<INodeless> {


            let collect = db.collection::<INodeless>(LEDGER_BIT);
            let filter = doc!{ "session" : self.session.to_owned()};
                let update_doc = doc! {
                    "$set" : {
                        "lid" : self.lid.clone(),
                        "remaining" : self.remaining,
                        "status" : bson::to_bson(&self.status).unwrap(),
                        "email" : self.email.clone(),
                    },
                };

                let update_opts = FindOneAndUpdateOptions::builder().return_document(mongodb::options::ReturnDocument::After).build();
                if let Ok(content) = collect.find_one_and_update(filter, update_doc, update_opts).await{

                    return content;
                }

                Some(INodeless{ amount: 0, email: "".to_string(), status: TransactionStatus::Expire, remaining: 0.00, name: "".to_string(), session: "".to_string(), lid: "".to_string() })
        }


        /// store inovice allow to return inovice generated by store. 
        pub async fn store_inovice(&mut self, node : &Nodeless) -> Result<nodeless_rs::store::Invoice, nodeless_rs::error::NodelessError> {

                node.get_store_invoice(STORE, &self.lid).await
        }

        /// store status allow to tell what is the status of your transaction .
        pub async fn store_status(&mut self, node : &Nodeless) -> Result<nodeless_rs::store::InvoiceStatus, nodeless_rs::error::NodelessError> {

                node.get_store_invoice_status(STORE, &self.lid).await
        }
        
        // find transaction record in our database
        async fn find_tnx_record(&mut self , db : Database) -> std::io::Result<INodeless> {

            let collection = db.collection::<INodeless>(LEDGER_BIT);
            let mut inode = INodeless{ amount: 0, email: "".to_string(), status: TransactionStatus::Process, remaining: 0.00, name: "".to_string(), session: "".to_string(), lid: "".to_string() };

            let filter = doc!{ "session" : self.session.to_owned()};
                let find_opts = FindOptions::builder().sort(doc!{ "session" : 1}).build();

                let mut cursor = collection.find(filter, find_opts).await.unwrap();
            
                while let Some(record) = cursor.try_next().await.unwrap(){

                    if record.session.is_empty(){

                        panic!("Make sure your query is not empty");
                    }

                    inode = record;
                }

                Ok(inode)
                
        }


        /// get tnx record return record ; if certain information provide under certain rules.
        pub async fn get_tnx_record(&mut self, db : Database) -> INodeless {

            
            let Ok(record) = self.find_tnx_record(db.to_owned()).await else { return INodeless{amount : 0, email : "".to_string(), status : TransactionStatus::Process, remaining : 0.00, name : "".to_string(), session : "".to_string(), lid: "".to_string()} };

            record
        }


        /// get store tnx return transactions happen ... if no transaction happened then it will return empty transaction.
        pub async fn get_store_tnx(&mut self, node : &Nodeless) -> Vec<Transaction>{

           let mut store_tx : Vec<Transaction> = Vec::<Transaction>::new(); 
           let tnx = node.get_transactions(true).await;
            if let Ok(tx) = tnx{

                if tx.is_empty(){

                    println!("No transaction yet made from your account! {:?}", tx);
                }else{

                    store_tx.push(tx[0].to_owned().clone());
                }

                
            }

            store_tx
        }
    }

    

}

