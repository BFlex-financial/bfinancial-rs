
async fn test_card(){
    let client = bfinancial_rs::Client::login("admin");
    let payments = client.payments;
    let x = payments.create(payment::PaymentCreate::Card(payment::CardCreate {
        amount: 0.2,
        payer_email: "edsuz.contato@gmail.com".into(),
        number:"5031433215406351".into(),
        cvv:"123".into()
    })).await;

    match x {
        Ok(pix) => println!("{:#?}", pix),
        Err(falha) => println!("Falhou em {}", falha)
    }
}


use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate} };

async fn test_pix(){
    let client = Client::login("admin");
    let payments = client.payments;
    let payment_data = payments.create(
        PaymentCreate::Pix(payment::PixCreate {
            amount: 0.2,
            payer_email: "lucasdwbfff@gmail.com".into()
        })
    ).await;

    match payment_data {
        Ok(pix) => println!("{:#?}", pix),
        Err(msg) => println!("Error returned when generating payment: {}", msg)
    }
}

#[tokio::main]
async fn main() {
    test_pix().await;
}
