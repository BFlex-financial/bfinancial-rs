use std::{any::Any, os::windows::thread};
use serde::de::IntoDeserializer;
use serde_json::Value;
use tokio::runtime::Runtime;

use crate::Client;

#[derive(Clone, Debug, PartialEq)]
pub struct Pix {
  pub payment_id: String,
  pub qr_code: String,
  pub literal: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
  pub payment_id: String,
  pub total_amount: f64,
  pub increase: f64
}

#[derive(PartialEq, Debug, Clone)]
pub enum Response {
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

impl Response {
  pub fn access<T: 'static>(&self) -> Option<&T> {
    match self {
      Response::Card(card) => card.as_any().downcast_ref::<T>(),
      Response::Pix(pix) => pix.as_any().downcast_ref::<T>(),
    }
  }

  pub async fn check(&self, info: (Client, &'static str)) -> Result<(), String> {
    let id: String = match self {
      Response::Card(_) => {
        let payment = self.access::<Card>().unwrap();
        payment.payment_id.clone()
      },
      Response::Pix(_) => {
        let payment = self.access::<Pix>().unwrap();
        payment.payment_id.clone()
      }
    };

    let mut start: String = String::new();

    let client = reqwest::Client::new();
    let res = client.get(format!("{}/payment/get", info.0.payments.__api.clone()))
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
      return Err("Payment not found".into());
    }

    std::thread::spawn(move || {
        let rt: Runtime = Runtime::new().unwrap();
        let mut rerun: bool = true;

        let result = rt.block_on(async {
          while rerun {

            let client = reqwest::Client::new();
            let res = client.get(format!("{}/payment/get", info.0.payments.__api.clone()))
              .header("Authorization-key", info.0.payments.__auth.clone())
              .header("Content-Type", "application/json")
              .body(format!("{{\"payment_id\":{}}}", id))
              .send()
              .await
              .unwrap();

            let response: Value = res.json::<Value>().await.unwrap();

            if let Some(error) = response.get("data").and_then(|data| data.get("error")) {
              return Err(error.as_str().unwrap().to_string());
            }

            if let Some(status) = response.get("data").and_then(|data| data.get("status")) {
              if status.as_str().unwrap() != start && status.as_str().unwrap() != info.1 {
                return Err(format!("Received the '{}' status, but has expected '{}'.", status.as_str().unwrap(), info.1))
              }

              if status.as_str().unwrap() == info.1 {
                rerun = false;
              }
            }

            std::thread::sleep(std::time::Duration::new(5,0));

          }

          Ok(())
        });

        result
    })
    .join()
    .unwrap() 
    
  }
}