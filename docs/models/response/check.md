# Check payment status


 - This function checks the payment status in real time.

The system waits for any update on the payment status.
If the status changes from X to Y, an update will be reported. 
If the status is as expected, the system will return "Ok". 
If the status is different from what is expected, 
an error ("Err") will be returned.

# Exemple

```rust
// Create the payment
let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(payment::PixCreate { ... })).await;
 
// Ensures that there are no errors when creating the payment
if let Err(fail) = &payment { ... }

/* 
  Wait for the payment status to be updated. 
  If it is updated to `approved`, it should
  fall on Ok. If it changes to any other
  status, it should fall on Err, returning
  the status that was obtained.
*/
match
  payment.check((client, "approved")).await
{
  Ok(_) => println!("Payment approved"),
  Err(msg) => println!("Ocurred a error: {msg}") 
}
```