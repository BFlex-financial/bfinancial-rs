/**
 * By. Lucas Silveira <contato.lucasdwbfff@gmail.com>
 */

use reqwest;
use serde_json::Value;

pub(crate) mod utils;
pub mod models;
use models::{client::payment::PaymentCreate, server::payment::{Card, Pix, Response, Status}};

#[derive(Clone, Debug)]
pub struct Client {
  pub auth: String,
  pub payments: Payments
}

impl Client {
  /// # Login
  /// 
  /// Enter your BFlex Financial Solutions access code.
  /// Here we will save important information about your account.
  ///
  /// # _DO NOT SHARE THIS KEY WITH ANYONE!_
  pub fn login(auth: &'static str) -> Self {
    let payments = Payments::call(format!("Bearer {auth}"));
    
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
  pub(crate) __api: String,
  pub(crate) __auth: String,
  __alocation: bool
}

impl Payments {
  pub(crate) fn call(auth: String) -> Payments {
    Payments {
      __api: "http://127.0.0.1:8080".into(),
      __auth: auth,
      __alocation: true
    }
  }

  /// # Create payments
  /// 
  /// This function creates a payment using BFlex Financial Solutions
  /// 
  /// # Examples
  /// 
  /// ```rust
  /// let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(PixCreate {
  ///   amount: 1000.00,
  ///   payer_email: "test@gmail.com".into(),
  ///   payer_cpf: "12345678910".into()
  /// })).await;
  /// 
  /// assert!(payment, Ok(Response::Pix({ 
  ///   payment_id: 0, 
  ///   qr_code: String::new(),
  ///   literal: String::new()
  /// }));
  /// ```
  pub async fn create(&self, data: PaymentCreate) -> Result<Response, String> {
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
      
    match response.clone().get("data").unwrap().get("error") {
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
              status: Status::Pending,
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
              status: Status::Pending,
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

  /// # Get payment information
  /// 
  /// To collect payment data, you need to use the `obtain` function.
  /// 
  /// This way, we can collect all the information necessary for your system to work.
  /// 
  /// ## You can:
  /// 
  /// - Collect the payment qr_code again
  /// - Collect the transaction status
  /// - Collect the cause of the failure (if it fails)
  /// - _Among other data..._
  /// 
  /// # Exemple
  /// 
  /// ```rust
  /// // Create the payment
  /// let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(payment::PixCreate { ... })).await;
  /// 
  /// // Ensures that there are no errors when creating the payment
  /// if let Err(fail) = &payment {
  ///    println!("Error returned when generating payment: {}", fail);
  /// }
  /// 
  /// // Collects PIX from within the enumerator
  /// let pix: &Pix = payment.access::<Pix>().unwrap();
  /// 
  /// // Collects and prints payment data
  /// let info = payments.obtain(&pix.payment_id).await.unwrap();
  /// println!("{:#?}", info);
  /// 
  /// ```
  pub async fn obtain(&self, id: impl ToString) -> Result<Response, String> {
    let client = reqwest::Client::new();

    let res = client.get(format!("{}/payment/get", self.__api))
      .header("Authorization-key", self.__auth.clone())
      .header("Content-Type", "application/json")
      .body(format!("{{\"payment_id\":{}}}", id.to_string()))
      .send()
      .await
      .unwrap();

    let response: Value = res.json::<Value>().await.unwrap();
    let api_response: Value = response.clone();
    let data: &Value = api_response.get("data").unwrap();

    let status: Status = utils::status::transform(
      response
        .clone()
        .get("data")
        .unwrap()
        .clone()
        .get("status")
        .unwrap()
        .as_str()
        .unwrap()
        .into(),
      response
        .clone()
        .get("data")
        .unwrap()
        .clone()
        .get("cause")
        .unwrap()
        .as_str()
        .unwrap()
        .into()
    );

    match response.clone().get("data").unwrap().get("error") {
      Some(error) => {
        return Err(error.as_str().unwrap().to_string());
      } 
      None => {}
    }

    match response.clone().get("data").unwrap().get("method") {

      Some(x) if x.as_str().unwrap() == "Pix" => Ok(Response::Pix(Pix {
        status,
        payment_id: id.to_string(),
        literal: data
          .clone()
          .get("payment_info")
          .unwrap()
          .get("literal")
          .unwrap()
          .as_str()
          .unwrap()
          .into(),
        qr_code: data
          .clone()
          .get("payment_info")
          .unwrap()
          .get("qr_code")
          .unwrap()
          .as_str()
          .unwrap()
          .into()
      })),

      Some(x) if x.as_str().unwrap() == "Card" => Ok(Response::Card(Card {
        status,
        payment_id: id.to_string(),
        increase: data
          .clone()
          .get("payment_info")
          .unwrap()
          .get("increase")
          .unwrap()
          .as_f64()
          .unwrap(),
        total_amount: data
          .clone()
          .get("payment_info")
          .unwrap()
          .get("total_amount")
          .unwrap()
          .as_f64()
          .unwrap()
      })),

      None | Some(_) => Err("Unknown payment method".into()),

    }
    
  }
}
