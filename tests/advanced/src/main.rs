use bfinancial_rs;
use bfinancial_rs::models::client::payment;
use tokio;

async fn test_pix(){
    let client = bfinancial_rs::Client::login("admin");
    let payments = client.payments;
    let payment_data = payments.create(payment::PaymentCreate::Pix(payment::PixCreate {
        amount: 0.2,
        payer_email: "test@gmail.com".into()
    })).await;

    match payment_data {
        Ok(pix) => println!("{:#?}", pix),
        Err(falha) => println!("Falhou em {}", falha)
    }
}
async fn test_card(){
    let client = bfinancial_rs::Client::login("admin");
    let payments = client.payments;
    let payment_data = payments.create(payment::PaymentCreate::Card(payment::CardCreate {
        amount: 22.0,
        payer_email: "test@gmail.com".into(),
        payer_cpf: "12345678909".into(),
        payer_name: "CONT".into(),
        expiration_month: 11,
        expiration_year: 2025,
        number:"5031433215406351".into(),
        cvv:"123".into()
    })).await;

    match payment_data {
        Ok(pix) => println!("{:#?}", pix),
        Err(falha) => println!("Falhou em {}", falha)
    }
}




#[tokio::main]
async fn main() {
    // test_pix().await;
    test_card().await;
}
