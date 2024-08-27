use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Pix {
  pub amount: f64,
  pub payer_email: String
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Card {
  pub payer_email: String,
  pub amount: f64,
  pub number: String,
  pub cvv: String 
}

#[derive(PartialEq)]
pub enum Payment {
  Card(Card),
  Pix(Pix)
}