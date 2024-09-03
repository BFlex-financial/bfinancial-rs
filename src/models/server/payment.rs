use std::any::Any;

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
}