use serde::{ Serialize, Deserialize };

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PixCreate {
  pub amount: f64,
  pub payer_email: String
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CardCreate {
  // pub installments: usize,
  pub amount: f64,
  pub payer_email: String,
  pub number: String,
  pub cvv: String 
}


#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "method")]
pub enum PaymentCreate {
  Card(CardCreate),
  Pix(PixCreate)
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PaymentReceived {
  pub payment_id: usize
}