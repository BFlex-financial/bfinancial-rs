# Get payment information

To collect payment data, you need to use the `obtain` function.

This way, we can collect all the information necessary for your system to work.

## You can:

 - Collect the payment qr_code again
 - Collect the transaction status
 - Collect the cause of the failure (if it fails)
 - _Among other data..._
   
# Exemple
   
```rust
// Create the payment
let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(payment::PixCreate { ... })).await;

// Ensures that there are no errors when creating the payment
if let Err(fail) = &payment {
  println!("Error returned when generating payment: {}", fail);
}

// Collects PIX from within the enumerator
let pix: &Pix = payment.access::<Pix>().unwrap();

// Collects and prints payment data
let info = payments.obtain(&pix.payment_id).await.unwrap();
println!("{:#?}", info);
```