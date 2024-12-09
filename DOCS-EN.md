<img align="right" src="https://imgur.com/EtCvGVc.png" height="85">

🦀 Library to assist with large and small scale payments

# BFlex Rust SDK
> [!TIP] 
> Needs support with something from the SDK? You can try interacting in [our Discord community](https://discord.gg/cdEnEtwehC)

Summary
==========================================

  <!--Table of indexes-->
  * [Features](#features)
  * [Installation](#installation)
    * [Requirements](#requirements)
  * [Let's get started](#let's-get-started)
  * [Code Examples](#examples)

## Features

**Ease of SDK Implementation**: All of our SDKs are designed to maintain a consistent structure of identifiers and usage patterns across different programming languages. This provides an extremely intuitive integration: even if you do not have in-depth knowledge of the specific language, you will be able to implement the SDK with ease.

**Ease of Obtaining Results**: Unlike other payment platforms, with BFlex you can create a payment using your preferred method with just a few lines of code. And the best: all this in a secure way, without the need to manage direct communication with the consumer.

**Checkout User Interface**: If your project does not require the user to stay on a specific platform and allows redirects, you can simplify the server-side implementation by simply redirecting the user to an official checkout page URL. from BFlex, ensuring practical and efficient integration.

![Checkout page img](https://imgur.com/Y3o7FJ2.png)
<div align="center">

###### Placeholder image
</div>

## Installation

### Requirements

  * Cargo 1.7 / Rust 1.2 (or higher)

### Package installation

To start, add the library from BFlex to your project. In the `cargo.toml` file, insert the following dependency:

```toml
[dependencies]
bfinancial_rs = "*"
```

Then, use the **[🦀 Rust](https://rust-lang.org/)** to download the library. This can be done with the command:

```sh-session
$ cargo install bfinancial_rs
```

## Let's get started

### 1. Initial setup

Use the **Client** class from the SDK to log in with your **API key**. After logging in, you will have access to the pre-configured instance of the Payments class, which is automatically returned by the **Client** class.

```rust
use tokio;
use bfinancial_rs::Client;

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;al payments = client.payments;
}
```

### 2. Make your first payment!

Try the integration by making a test payment of 1 **BRL**. The amount will be credited to your **BFlex** account via a ** Pix** generated automatically by the SDK!

```rust
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Pix };

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into()
    amount:       1.0,
  })).await;

  match payment_data {
    Ok(pix)   => println!("{:#?}", pix.access::<Pix>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}
```

### 3. Documentation

You can see the [📚 **Documentation** by clicking here](https://bflex.tech/docs/rust-sdk).

## Examples

### Generating payments with PIX 
```rust
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Pix };

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into()
    amount:       1000.0,
  })).await;

  match payment_data {
    Ok(pix)   => println!("{:#?}", pix.access::<Pix>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}
```

### Generating Card Payments 

```rust
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Card };

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Card(payment::CardCreate {
    expiration_month: 11,
    expiration_year:  2025,
    payer_email:     "test@gmail.com".into(),
    payer_name:      "test user".into(),
    payer_cpf:       "12345678909".into(),
    number:          "5031433215406351".into(),
    amount:           1000.0,
    cvv:             "123".into()
  })).await;


  match payment_data {
    Ok(card)  => println!("{:#?}", card.access::<Card>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}

```

### Collecting payment data

```rust
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into(),
    payer_cpf:   "12345678909".into()
    amount:       1000.0,
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

Or, if you don't know the exact type of payment you're dealing with, you can use:

```rust
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::{Response, Pix}}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into(),
    payer_cpf:   "12345678909".into()
    amount:       1000.0,
  })).await;

  if let Err(fail) = &payment_data {
    println!("Error returned when generating payment: {}", fail);
  }

  let payment = payment_data.clone().unwrap();

  match payment {
    Response::Card(card) => {
      let collected = payments.obtain(&card.payment_id).await.unwrap()
      println!("{:#?}", collected);
    }

    Response::Pix(pix) => {
      let collected = payments.obtain(&pix.payment_id).await.unwrap()
      println!("{:#?}", collected);
    }
  }
  
}
```

### Real-time Status Validation

With this, you can wait to receive a Status, and know if it was received, or another one.

```rust
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = &client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into(),
    payer_cpf:   "12345678909".into()
    amount:       1000.0,
  })).await;

  if let Err(fail) = &payment_data {
    println!("Error returned when generating payment: {}", fail);
  }

  let payment = payment_data.unwrap();
  match
    payment.check((client, "approved")).await
  {
    Ok(_) => println!("Payment approved"),
    Err(msg) => println!("Ocurred a error: {msg}") 
  }
}
```