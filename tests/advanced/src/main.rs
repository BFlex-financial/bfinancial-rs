use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
    let client = Client::login("admin");
    let payments = &client.payments;
    let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
        amount: 0.02,
        payer_email: "test@gmail.com".into(),
        payer_cpf: "12345678910".into()
    })).await;

    if let Err(fail) = &payment_data {
        println!("Error returned when generating payment: {}", fail);
    }

    let payment = payment_data.clone().unwrap();
    let pix: &Pix = payment.access::<Pix>().unwrap();
    println!("Pix copia e cola: {}", pix.literal);
    let collected = payments.obtain(&pix.payment_id).await.unwrap();
    println!("{:#?}", collected);
    match
        payment.check((client, "approved")).await
    {
        Ok(_) => println!("Payment approved"),
        Err(msg) => println!("Ocurred a error: {msg}") 
    }
}
