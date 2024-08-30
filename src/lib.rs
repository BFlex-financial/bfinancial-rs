use reqwest;
use serde_json::Value;

pub mod models;
use models::client::payment::PaymentCreate;


#[derive(Clone, Debug)]
pub struct Client {
  pub auth: String,
  pub payments: Payments
}

impl Client {
  pub fn login(auth: &'static str) -> Self {
    let payments = Payments::call(auth.into());
    
    Self {
      auth: auth.into(), payments
    }
  }
  
}

/*
  Payment struct implementation
*/

#[derive(Clone, Debug)]
pub struct Payments {
  __api: String,
  __auth: String,
  __alocation: bool
}

impl Payments {
  pub fn call(auth: String) -> Payments {
    Payments {
      __api: "http://127.0.0.1:8080".into(),
      __auth: auth,
      __alocation: true
    }
  }

  pub async fn create(&self, data: PaymentCreate) -> Result<models::server::payment::Response, String> {
    let client = reqwest::Client::new();


    let res = client.post(format!("{}/payment/create", self.__api))
          .header("Authorization-key", self.__auth.clone())
          .header("Content-Type", "application/json")
          .body(
            serde_json::to_string(&data).unwrap()
          )
          .send()
          .await
          .unwrap();

    let response: Value = res.json::<Value>().await.unwrap();
      
    match response.clone().get("error") {
        Some(error) => {
          return Err(error.as_str().unwrap().to_string());
        } 
        None => {}
    }
    match data {
      
      /*
        Payment with credit/debit card
      */
      PaymentCreate::Card(_) => {
          Ok(
            models::server::payment::Response::Card(
              models::server::payment::Card {
                payment_id: 
                          response.clone()
                          .get("data")
                          .unwrap()
                          .get("payment_id")
                          .unwrap()
                          .as_i64()
                          .unwrap()
                          .to_string(),
              total_amount: 
                          response.clone()
                          .get("data")
                          .unwrap()
                          .get("total_amount")
                          .unwrap()
                          .as_f64()
                          .unwrap(),
              increase: 
                          response.clone()
                          .get("data")
                          .unwrap()
                          .get("increase")
                          .unwrap()
                          .as_f64()
                          .unwrap()
              }
            )
          )
        },

      /*
        Payment with PIX
      */
      PaymentCreate::Pix(_) => {
        Ok(
          models::server::payment::Response::Pix(
            models::server::payment::Pix {
              payment_id: 
                        response.clone()
                        .get("data")
                        .unwrap()
                        .get("payment_id")
                        .unwrap()
                        .as_i64()
                        .unwrap()
                        .to_string(),
              
              qr_code: 
                      response.clone()
                      .get("data")
                      .unwrap()
                      .get("qr_code")
                      .unwrap()
                      .get("base64")
                      .unwrap()
                      .as_str()
                      .unwrap()
                      .into(),
                    
              literal: 
                      response.clone()
                      .get("data")
                      .unwrap()
                      .get("qr_code")
                      .unwrap()
                      .get("literal")
                      .unwrap()
                      .as_str()
                      .unwrap()
                      .into(),
            }
          )
        )
      }
    }
  }
}
