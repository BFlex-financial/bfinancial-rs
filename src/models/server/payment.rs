use std::any::Any;
use serde_json::Value;

use crate::Client;

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
  Reject(String),
  Cancelled,
  Approved,
  Refunded,
  Pending,
  Uknown
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pix {
  pub status: Status,
  pub payment_id: String,
  pub qr_code: String,
  pub literal: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
  pub status: Status,
  pub payment_id: String,
  pub total_amount: f64,
  pub increase: f64
}

#[derive(PartialEq, Debug, Clone)]
pub enum Response {
  Checkout(String),
  Card(Card),
  Pix(Pix)
}

pub trait RespData {
  fn as_any(&self) -> &dyn Any;
}

impl RespData for Card {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl RespData for Pix {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl RespData for String {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

#[derive(Copy, PartialEq, Debug, Hash, Clone)]
pub enum Verified<T> {
  Fail(T),
  Success
}

impl Response {
  /// # Access the payment
  ///
  /// If you have created a payment, and you know exactly
  /// what type it is and you are sure that it does not
  /// need verification via MATCH, you can force direct 
  /// access with Access
  /// 
  /// # Examples
  /// 
  /// - If you do not know what type of payment was generated (in this case PIX), you should use:
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
  /// // It checks the type of payment and handles it appropriately for each
  /// match payment.unwrap() {
  ///   Response::Pix(pix) => { ... },
  ///   Response::Card(card) => { ... },
  /// }
  /// ```
  /// 
  /// - If you know what type of payment was generated and want to save lines of code, you should use:
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
  /// ```
  pub fn access<T: 'static>(&self) -> Option<&T> {
    match self {
      Response::Checkout(checkout) => checkout.as_any().downcast_ref::<T>(),
      Response::Card(card) => card.as_any().downcast_ref::<T>(),
      Response::Pix(pix) => pix.as_any().downcast_ref::<T>(),
    }
  }

  /// # Check payment status
  /// 
  /// - This function checks the payment status in real time.
  /// 
  /// The system waits for any update on the payment status.
  /// If the status changes from X to Y, an update will be reported. 
  /// If the status is as expected, the system will return "Ok". 
  /// If the status is different from what is expected, 
  /// an error ("Err") will be returned.
  /// 
  /// # Exemple
  /// 
  /// ```rust
  /// // Create the payment
  /// let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(payment::PixCreate { ... })).await;
  /// 
  /// // Ensures that there are no errors when creating the payment
  /// if let Err(fail) = &payment { ... }
  /// 
  /// /* 
  ///   Wait for the payment status to be updated. 
  ///   If it is updated to `approved`, it should
  ///   fall on Ok. If it changes to any other
  ///   status, it should fall on Err, returning
  ///   the status that was obtained.
  /// */
  /// match
  ///   payment.check((client, "approved")).await
  /// {
  ///   Ok(_) => println!("Payment approved"),
  ///   Err(msg) => println!("Ocurred a error: {msg}") 
  /// }
  /// ```
  pub async fn check(&self, info: (Client, &'static str)) -> Verified<String> {
    use Verified::*;
    
    let id: String = match self {
      Response::Checkout(_) => unreachable!(),
      Response::Card(payment) => payment.payment_id.clone(),
      Response::Pix(payment) => payment.payment_id.clone()
    };

    let mut start: String = String::new();

    let client = reqwest::Client::new();
    let res = client.get(format!("{}/api/payment/get", info.0.payments.__api.clone()))
      .header("Authorization-key", info.0.payments.__auth.clone())
      .header("Content-Type", "application/json")
      .body(format!("{{\"payment_id\":{}}}", id))
      .send()
      .await
      .unwrap();

    let response: Value = res.json::<Value>().await.unwrap();

    if let Some(status) = response.get("data").and_then(|data| data.get("status")) {
      start = status.as_str().unwrap().to_string();
    } else {
      return Fail("Payment not found".into());
    }

    let mut rerun: bool = true;

    while rerun {

      let client = reqwest::Client::new();
      let res = client.get(format!("{}/api/payment/get", info.0.payments.__api.clone()))
        .header("Authorization-key", info.0.payments.__auth.clone())
        .header("Content-Type", "application/json")
        .body(format!("{{\"payment_id\":{}}}", id))
        .send()
        .await
        .unwrap();

      let response: Value = res.json::<Value>().await.unwrap();

      if let Some(error) = response.get("data").and_then(|data| data.get("error")) {
        return Fail(error.as_str().unwrap().to_string());
      }

      if let Some(status) = response.get("data").and_then(|data| data.get("status")) {
        if status.as_str().unwrap() != start && status.as_str().unwrap() != info.1 {
          return Fail(format!("Received the '{}' status, but has expected '{}'.", status.as_str().unwrap(), info.1))
        }

        if status.as_str().unwrap() == info.1 {
          rerun = false;
        }
      }

      std::thread::sleep(std::time::Duration::new(5,0));

    }

    Success

  }
}