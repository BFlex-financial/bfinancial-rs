# Access the payment


If you have created a payment, and you know exactly
what type it is and you are sure that it does not
need verification via MATCH, you can force direct 
access with Access

# Examples
 
 - If you do not know what type of payment was generated (in this case PIX), you should use:

```rust
// Create the payment
let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(payment::PixCreate { ... })).await;
 
// Ensures that there are no errors when creating the payment
if let Err(fail) = &payment {
  println!("Error returned when generating payment: {}", fail);
}

// It checks the type of payment and handles it appropriately for each
match payment.unwrap() {
  Response::Pix(pix) => { ... },
  Response::Card(card) => { ... },
}
```

 - If you know what type of payment was generated and want to save lines of code, you should use:

```rust
// Create the payment
let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(payment::PixCreate { ... })).await;

// Ensures that there are no errors when creating the payment
if let Err(fail) = &payment {
  println!("Error returned when generating payment: {}", fail);
}

// Collects PIX from within the enumerator
let pix: &Pix = payment.access::<Pix>().unwrap();
```