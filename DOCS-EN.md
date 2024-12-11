<img align="right" src="https://imgur.com/EtCvGVc.png" height="85">

🦀 Library to assist with large and small scale payments

# BFlex Rust SDK

> [!TIP]
> Need support with the SDK? You can try interacting in [our Discord community](https://discord.gg/cdEnEtwehC)

Table of Contents
=========================================

  <!--Table of contents-->
  * [Features](#features)
  * [Installation](#installation)
    * [Requirements](#requirements)
  * [Getting Started](#getting-started)
  * [Code Examples](#examples)

## Features

**Easy SDK Integration**: All our SDKs are designed to maintain a consistent structure of identifiers and usage patterns across different programming languages. This provides an extremely intuitive integration: even if you don’t have deep knowledge of the specific language, you’ll be able to implement the SDK with ease.

**Easy Payment Handling**: Unlike other payment platforms, with BFlex you can create a payment with just a few lines of code using your preferred method. And best of all, it's secure, without the need to manage direct communication with the consumer.

**Checkout UI**: If your project doesn’t require the user to stay on a specific platform and allows redirects, you can simplify the server-side implementation. Just redirect the user to an official BFlex checkout page URL, ensuring a practical and efficient integration.

<!-- ![Checkout page img](https://imgur.com/Y3o7FJ2.png) -->

## Installation

### Requirements

  * Cargo 1.7 / Rust 1.2 (or higher)

### Installing the Package

To get started, add the BFlex library to your project. In the `cargo.toml` file, add the following dependency:

```toml
[dependencies]
bfinancial_rs = "*"
```

Then, use the **[🦀 Rust](https://rust-lang.org/)** SDK to download the library. This can be done with the command:

```sh-session
$ cargo install bfinancial_rs
```

## Getting Started


### 1. Initial Setup

Use the **Client** class from the SDK to log in with your **API key**. After logging in, you will have access to a pre-configured instance of the Payments class, which is automatically returned by the **Client** class.

```rust
use tokio;
use bfinancial_rs::Client;

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
}
```

### 2. Make Your First Payment!

Test the integration by making a test payment of 1 **USD**. The amount will be credited to your **BFlex** account via an automatically generated **Pix** payment by the SDK!

```rust
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Pix };

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into(),
    amount:      1.0,
  })).await;

  match payment_data {
    Ok(pix)   => println!("{:#?}", pix.access::<Pix>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}
```

### 3. Documentation

You can view the [📚 **Documentation** by clicking here](https://bflex.tech/docs/rust-sdk). 

## Examples

  * [Creating Payments](#creating-payments)
    * [Checkout Interface for Purchases](#checkout-interface-for-purchases) 
  * **Payment Utilities** 
    * [Collecting Payment Data](#collecting-payment-data) 
    * [Real-Time Status Check](#real-time-status-validation) 

## Creating Payments

To create a payment, collect the payments instance received after logging in through the SDK and use the `create` method inside it.

The `create` method accepts an `enum`, with the fields:

```rust
pub enum PaymentCreate {
  Pix(PixCreate),
  Card(CardCreate),
  Checkout(Checkout)
}
``` 

The field `Checkout(Checkout)` is special, simply due to its definition.
The most complete explanation about checkout can be found in the [**Checkout Interface**](#checkout-interface-for-purchases) section.

Every time we generate a payment, we receive a type `Future<Result<Response, String>>`, meaning we need to wait for the payment to be validated by the BFlex server and either receive a response containing the generated payment data or an error message. What this means:

Whenever we use `payments.create(PaymentCreate::Pix(PixCreate { ... }))`, we need to have an `await` to wait for the server’s response before continuing the code. And, of course, we need to check whether the status is a valid response or an error message. To validate if the response is positive or not, we can use an `if` statement that breaks the code flow in case there is any issue. Example:

```rust
fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(PixCreate { ... })).await;
  
  if let Err(err) = &payment {
    println!("Error: {err}");
    return;
  }

  // ...
}
```

We can also capture the error using Rust’s `match` keyword, like this:

```rust
match payment {
  Err(err) => {}
  Ok(data) => {}
}
```

Now, we know how to parse the error, but what about the response?

The response can be parsed in different ways. If you used the `if` statement, you can simply add `.unwrap()` below it, since the code will only reach that point if the payment was successfully created. However, now we have an `enum`, named `Response`, in our hands. To access the data inside this enumerator, we can use:

```rust
let data: Response = payment.unwrap();

match data {
  Response::Pix(pix) => {},
  Response::Card(card) => {},
  Response::Checkout(checkout) => {}
}
```

Or, if you know exactly which payment type you're dealing with (like a block of code that runs only when making a PIX payment), you can use the `access` method:

> [!TIP]
> Access in a `PIX` or `Card` payment returns `struct`s. For Checkout, it is a `String`.

```rust
let data: Response = payment.unwrap();
let pix = data.access::<Pix>().unwrap(); 
```

## Checkout Interface (Purchases)

As mentioned earlier, Checkout has a few different properties. While other payment methods use simple `struct`s containing the buyer’s details and basic payment information, Checkout involves more complexities and uses some different mechanisms for completing the sale.

The `Checkout` struct contains some special fields, like `amount` and `products`.

- **Amount**: Although the `amount` field is present in all `struct`s, it works differently here. It must include the price of cataloged products (with affiliation). If you are reselling a product (either public or private, as long as it’s not yours), the product price must be 100% included in the `amount` field of the checkout, and you can inflate the price of the products. The extra amount will go to your BFlex Wallet. Example:

- Product 1: Automotive windshield cleaner **$20.00**
- Product 2: Digital multimeter **$10.00**

If these products are resold, the minimum allowed in `amount` would be **$31.00**, ensuring at least $1.00 from the sale goes to your Wallet.

For example, if the same products were sold for **$60.00**, the exact same products would have the prices: `USD 40.00` and `USD 20.00`.

- **Products**: The `products` field is a `Vec<Product>`, as expected. However, the `enum` _Product_ has some special mechanisms. These are:

### Custom Products

Custom products are 100% created via code. You simply pass a `struct` _CustomProduct_ into `Product::Custom()`, containing all product data, such as:

- `price`: Price;
- `description`: Description;   
- `name`: Name;   
- `thumbnail`: Image;   

The prices don't necessarily need to match the `amount` field, because if the product is being created, it belongs to you. The responsibility for the final price lies with you as the seller.

### Cataloged Products

Cataloged products are pre-created in the BFlex product panel. BFlex offers both public and private products. Public products are available for anyone to sell, with a margin added to the product’s price. Example:

- A product costs **$200.00**, and you intend to sell it. You must charge more than $200.00, so the supplier gets the product price and you get your margin.

In the following example, we are making **$40.00** profit on a **public product** priced at $200.00:

```rust
let payment: Result<Response, String> = payments.create(PaymentCreate::Checkout(Checkout {
  amount: 240.00,
  products: vec![
    Product::Caloged(CatalogedProduct {
      product_id: "200.00 product ID".into(),
      affiliation: Affiliation::Yes(ProductType::Public)
    })
  ]
})).await;
```

> As shown, the API determines whether the product is public or private by checking if affiliation is `true`. You can see an example of [Non-affiliation here](#cataloged-products-without-affiliation).

For a **private product**, you must first have a certificate linked to your BFlex account. This certificate is what authorizes the sale of the product. The certificate must be issued by the product vendor and passed to you.

In the example below, we are making **$40.00** profit on a **private product** priced at $200.00:

```rust
let payment: Result<Response, String> = payments.create(PaymentCreate::Checkout(Checkout {
  amount: 240.00,
  products: vec![
    Product::Caloged(CatalogedProduct {
      product_id: "200.00 product ID".into(),
      affiliation: Affiliation::Yes(ProductType::Private(
        Authorization::cert("Certificate of permission to resell the product.")
      ))
    })
  ]
})).await;
```

### Cataloged Products (Without Affiliation)

Products without affiliation are pre-created products that belong to you. No additional information is needed other than the product ID and the `affiliation` set to `Affiliation::No`.

As with custom products, you have full control over the final price of the checkout. Prices don’t need to be charged, as you determine the final sale price, even if it’s $0.01.

### Collecting Payment Data

Payment data is protected and accessible only with the API key of the payment creator. Possible error responses for payment collection failures are:

- `Payment not found • 404`: The payment with ID `x` 
(Provided as the first argument in the `obtain` method) 
does not exist or could not be found.

- `Unauthorized access • 401`: The payment with ID `x` 
(Provided as the first argument in the `obtain` method) 
was found, but the creator of the payment is not you.

In the following example, we assume you have a previously created **PIX** payment and, for some reason, wish to collect its data, like the status and QR code information:

```rust
let pix: &Pix = payment.access::<Pix>().unwrap();
let collected = payments.obtain(&pix.payment_id).await.unwrap();
```

### Real-Time Status Validation

Using the `check` method, you can verify the status of a payment, checking whether it is pending or has been paid. On success, the response comes as an inverted `Option`. We call this `Verified<String>`.

> [!TIP]
> Import _Verified_ with `Verified::{self, *}`

```rust
match
  payment.check((client, "approved")).await
{
  Success => println!("Payment approved"),
  Fail(msg) => println!("Occurred an error: {msg}") 
}
```

If you don’t want to wait for the payment status to change, you can open a thread and use `check` in a second thread.

You can do this using Rust’s standard thread library and the `mpsc` channel to communicate between the thread and the main code (Optional tip). Like this example:

```rust
thread::spawn(move || {
  match
    payment.check((client, "approved")).await
  {
    Success => println!("Payment approved"),
    Fail(msg) => println!("Occurred an error: {msg}")
  }
});
``` 