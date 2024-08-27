use bfinancial_rs;
use bfinancial_rs::models::client::payment;
use tokio;

async fn testPix(){
    let client = bfinancial_rs::Client::start("admin");
    let payments = client.payments;
    let x = payments.new(payment::PaymentCreate::Pix(payment::PixCreate {
        amount: 0.2,
        payer_email: "lucasdwbfff@gmail.com".into()
    })).await;

    match x {
        Ok(pix) => println!("{:#?}", pix),
        Err(falha) => println!("Falhou em {}", falha)
    }
}
async fn testCard(){
    let client = bfinancial_rs::Client::start("admin");
    let payments = client.payments;
    let x = payments.new(payment::PaymentCreate::Card(payment::CardCreate {
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




#[tokio::main]
async fn main() {
    testPix().await;
    // testCard().await;
}
