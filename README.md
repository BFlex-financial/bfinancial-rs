# bfinancial-rs

# Gerando pagamentos via SDK
### Pix
```rs
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Pix };

async fn test_pix(){
  let client = Client::login("admin");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    amount: 22.0,
    payer_email: "test@gmail.com".into()
  })).await;

  match payment_data {
    Ok(pix) => println!("{:#?}", pix.access::<Pix>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}

#[tokio::main]
async fn main() {
  test_pix().await;
}
```

### Cartão
```rs
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Card };

async fn test_card(){
  let client = Client::login("admin");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Card(payment::CardCreate {
    amount: 22.0,
    payer_email: "test@gmail.com".into(),
    payer_cpf: "12345678909".into(),
    payer_name: "test user".into(),
    expiration_month: 11,
    expiration_year: 2025,
    number: "5031433215406351".into(),
    cvv: "123".into()
  })).await;

  match payment_data {
    Ok(card) => println!("{:#?}", card.access::<Card>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}

#[tokio::main]
async fn main() {
  test_card().await;
}
```