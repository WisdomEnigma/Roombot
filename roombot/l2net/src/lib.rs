

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


/// This module send & receive bitcoins over bitcoin network , there may be possible that lighting node doesn't work in your regions due to some international or national sansactions. But direct access to wallet is borderless exchange of currency.  

pub mod bitpayee{

    
    use bitcoins::{BitcoinMainnet,prelude::{Outpoint,Network, TxBuilder, ByteFormat}, enc::Address, types::{TxInput, TxOut}};
    

    // Bit Address is a private constant 
    const BITADDRESS : &str = "bc1quq4vz6wpnvydzqz2fhayh6pzgwmgjxxnu0x8yj";
    
    /// Bitenigma is a object which have two public & one private fields. Debug implementation is only on this object
    #[derive(Debug)]
    pub struct Bitenigma{

        /// sender [user who engage with our services] 
        pub sender : String,
        pub receiver : &'static str,
        price : u64,
    } 


    /// BitenigmaError enum hold various type of error that may be encounter during process.. 
    #[derive(Debug)]
    pub enum BitenigmaError{

        InvalidAddressIssue(&'static str),
        LowBalance(&'static str),
        InsufficientBalance(&'static str),
        EmptyBitAddress(&'static str),
        DuplicateAddress(&'static str),
        None
    }

    impl Bitenigma{


        /// Bitenigma used to send & received bitcoins over bitcoin network .. 
        ///```
        ///  asset_eq!(l2::bitpayee::Bitenigma::new("1cvfyuhf56e89444983456".to_string(), 20, Bitenigma{sender : "1cvfyuhf56e89444983456".to_string(), price :20});
        /// 
        /// ```text

        pub fn new(sender : String, price : u64) -> Bitenigma{

            Self{
                sender,
                receiver : BITADDRESS,
                price,
            }
        }


        /// this function check whether standard address provided or not
        /// 
        pub fn address_valid(&mut self) -> Result<BitenigmaError, BitenigmaError> {

            if self.sender.to_owned().to_string().is_empty(){

                return Err(BitenigmaError::EmptyBitAddress("This user have not provide bitcoin address"));
            }

            if self.sender.to_owned().to_string().eq(&self.receiver){

                return Err(BitenigmaError::DuplicateAddress("This address is not Allowed "));
            }

            if self.sender.to_owned().to_string().len().ge(&26) && self.sender.to_owned().to_string().len().ge(&35){

                return Err(BitenigmaError::InvalidAddressIssue("This address is invalid adddress"));
            } 

            return Ok(BitenigmaError::None);
        }


        /// pay_handshake is a special receiver which report who will pay the charges of a services, similar like invoice generated by bitcoin network , if your transaction process complete   
        ///```
        ///  let mut gateway = l2::bitpayee::Bitenigma::new("1cvfyuhf56e89444983456".to_string(), 20);
        ///  asset_ne!(pay_handshake().await, 1cvfyuhf56e89444983456)
        /// 
        ///```text
        pub async fn pay_handshake(&mut self) -> Address {

            let addr = BitcoinMainnet::string_to_address(&self.sender.to_owned()).unwrap();
            let tx = BitcoinMainnet::tx_builder();
            
            tx.version(1)
            .spend(Outpoint::default(), 0xffffffff)
            .pay(self.price, &addr)
            .insert_output(1, TxOut::default())
            .build()
            .unwrap()
            .serialize_hex();

            let decode_addr = BitcoinMainnet::decode_address(&addr);
            BitcoinMainnet::encode_address(&decode_addr).unwrap()
        }


        /// receiver handshake is an important function which report last transaction over bitcoin network.
        ///  
        pub async fn rece_handshake(&mut self) -> Address {

            let addr = BitcoinMainnet::string_to_address(self.receiver).unwrap();
            let tx = BitcoinMainnet::tx_builder();
            
            
            tx.version(1)
            .spend(Outpoint::default(), 0xffffffff)
            .insert_input(1, TxInput::default())
            .insert_output(2, TxOut::default())
            .build()
            .unwrap()
            .serialize_hex();

            let decoder = BitcoinMainnet::decode_address(&addr);
            BitcoinMainnet::encode_address(&decoder).unwrap()

        }


        /// both sender & receiver return whether transaction process complete by return same address.
        pub fn valid_sender(&mut self ,addr : Address) -> bool {

            addr.eq(&Address::Sh(self.sender.to_owned().to_string()))
        }

        pub fn valid_receiver(&mut self, addr : Address) -> bool {

            addr.eq(&Address::Sh(self.receiver.to_owned().to_string()))
        }
        
    }
}