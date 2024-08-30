use serde_json::Number;

#[derive(Clone, Debug, PartialEq)]
pub struct Pix {
  pub payment_id: String,
  pub qr_code: String,
  pub literal: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
  pub payment_id: Number,
  pub total_amount: f64,
  pub increase: f64
}

#[derive(PartialEq, Debug, Clone)]
pub enum Response {
  Card(Card),
  Pix(Pix)
}