use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
    let client = Client::login("admin");
    let payments = &client.payments;
    let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
        amount: 1.0,
        payer_email: "test@gmail.com".into(),
        payer_cpf: "12345678910".into()
    })).await;

    if let Err(fail) = &payment_data {
        println!("Error returned when generating payment: {}", fail);
    }

    let payment = payment_data.clone().unwrap();
    println!("Pix copia e cola: {}", payment.access::<Pix>().unwrap().literal);
    match
        payment.check((client, "approved")).await
    {
        Ok(_) => println!("Payment Aprooved"),
        Err(msg) => println!("Ocurred a error: {msg}") 
    }
}
