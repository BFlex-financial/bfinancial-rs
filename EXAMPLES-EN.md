<div align="center">
  <h1>Code Examples</h1>
</div>

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

### Card

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

#

## Payment Status Check

The status check works as follows: If checked and the status is PENDING, the payment will continue checking until there is any change in the status. When the status changes to any other state, we will receive some kind of response.

If it changes to the expected status from the CHECK, you will receive an Ok from the SDK.

If it changes to any other status, NOT the expected one, you will receive an error in Err.

```rs
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("admin");
  let payments = &client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    amount: 22.0,
    payer_email: "test@gmail.com".into(),
    payer_cpf: "12345678909".into()
  })).await;

  if let Err(fail) = &payment_data {
    println!("Error returned when generating payment: {}", fail);
  }

  let payment = payment_data.unwrap();
  match
    payment.check((client, "approved")).await
  {
    Ok(_) => println!("Payment approved"),
    Err(msg) => println!("Occurred an error: {msg}") 
  }
}
```

## Collecting Payment Data

To collect payment data, you need to use the `obtain` function. This allows you to gather all the necessary information for the operation of your system. You can:

- Collect the payment QR code again
- Collect the transaction status
- Collect the failure reason (if it failed)
- `Among other data...`

```rs
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("admin");
  let payments = &client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    amount: 22.0,
    payer_email: "test@gmail.com".into(),
    payer_cpf: "12345678909".into()
  })).await;

  if let Err(fail) = &payment_data {
    println!("Error returned when generating payment: {}", fail);
  }

  let payment = payment_data.clone().unwrap();
  let pix: &Pix = payment.access::<Pix>().unwrap();
  let collected = payments.obtain(&pix.payment_id).await.unwrap();
  println!("{:#?}", collected);
}
```