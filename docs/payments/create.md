# Create payments

This function creates a payment using BFlex Financial Solutions

 # Examples
 
```rust
let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(PixCreate {
  amount: 1000.00,
  payer_email: "test@gmail.com".into(),
  payer_cpf: "12345678910".into()
})).await;

assert!(matches!(
  payment, 
  Ok(Response::Pix({ 
    payment_id: 0, 
    qr_code: String::new(),
    literal: String::new()
  })
));
```