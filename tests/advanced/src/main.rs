use tokio;
use bfinancial_rs::{ models::{client::{payment::{Checkout, PaymentCreate}, product::{Affiliation, Authorization, CatalogedProduct, CustomProduct, Product, ProductType}}, server::payment::Response}, Client};

#[tokio::main]
async fn main() {
    let client = Client::login("admin");
    let payments = &client.payments;

    let payment: Result<Response, String> = payments.create(PaymentCreate::Checkout(Checkout {
        single_use: true,
        thumbnail: "https://img.jpg".into(),
        amount: 1729.32,
        title: "Teste".into(),
        description: "Produto de teste".into(),
        products: vec![
            Product::Custom(CustomProduct {
                description: "Produto A".into(),
                price: 200.00,
                thumbnail: "None".into(),
                name: "Product A".into()
            }),
            Product::Custom(CustomProduct {
                description: "MEU DEUS ME AJUDA FERNANDA FERNANDA FERNANDAFERNANDA SOCORRO".into(),
                price: 300.00,
                thumbnail: "None".into(),
                name: "Product ".into()
            }),
        ]
    })).await;

    if let Err(ref err) = payment {
        println!("Error: {}", err);
        return;
    }

    let checkout: Response = payment.unwrap();
    println!("URL: {}", checkout.access::<String>().unwrap());
}