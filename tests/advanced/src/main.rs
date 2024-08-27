use bfinancial_rs;
use bfinancial_rs::models::client::payment;
use tokio;

#[tokio::main]
async fn main() {
    let client = bfinancial_rs::Client::start("admin");
    let payments = client.payments;
    let x = payments.new(payment::Payment::Pix(payment::Pix {
        amount: 0.2,
        payer_email: "lucasdwbfff@gmail.com".into()
    })).await;

    match x {
        Ok(pix) => println!("{:#?}", pix),
        Err(falha) => println!("Falhou em {}", falha)
    }
}
