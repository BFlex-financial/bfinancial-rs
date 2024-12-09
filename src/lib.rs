/**
 * By. Lucas Silveira <contato.lucasdwbfff@gmail.com>
 */


pub(crate) mod utils;
pub mod models;
pub mod payments;
use payments::Payments;

#[derive(Clone, Debug)]
pub struct Client {
  pub auth: String,
  pub payments: Payments,
}

impl Client {
  /// # Login
  /// 
  /// Enter your BFlex Financial Solutions access code.
  /// Here we will save important information about your account.
  ///
  /// # _DO NOT SHARE THIS KEY WITH ANYONE!_
  pub fn login(auth: &str) -> Self {
    let payments = Payments::call(format!("Bearer {auth}"));
    
    Self {
      auth: auth.into(), payments
    }
  }
}

