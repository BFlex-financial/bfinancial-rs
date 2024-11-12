# bfinancial-rs

# Gerando pagamentos via SDK
### Pix
```rs
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Pix };

async fn test_pix(){
  let client = Client::login("admin");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    amount: 22.0,
    payer_email: "test@gmail.com".into()
  })).await;

  match payment_data {
    Ok(pix) => println!("{:#?}", pix.access::<Pix>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}

#[tokio::main]
async fn main() {
  test_pix().await;
}
```

### Cartão
```rs
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Card };

async fn test_card(){
  let client = Client::login("admin");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Card(payment::CardCreate {
    amount: 22.0,
    payer_email: "test@gmail.com".into(),
    payer_cpf: "12345678909".into(),
    payer_name: "test user".into(),
    expiration_month: 11,
    expiration_year: 2025,
    number: "5031433215406351".into(),
    cvv: "123".into()
  })).await;

  match payment_data {
    Ok(card) => println!("{:#?}", card.access::<Card>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}

#[tokio::main]
async fn main() {
  test_card().await;
}
```

#

## Verificação do status do pagamento

A verificação do check funciona da seguinte forma:
Se quando checkado, está PENDENTE, o pagamento ficará verificando até haver qualquer mudança nos status.
Quando status transacionar para qualquer outro, teremos algum tipo de retorno.

Se for alterado para o status esperado pelo CHECK, você receberá um Ok da SDK.

Se for alterado para qualquer outro status, SE NÃO o esperado, você receberá algum erro no Err.

```rs
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("admin");
  let payments = &client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    amount: 22.0,
    payer_email: "test@gmail.com".into(),
    payer_cpf: "12345678909".into()
  })).await;

  if let Err(fail) = &payment_data {
    println!("Error returned when generating payment: {}", fail);
  }

  let payment = payment_data.unwrap();
  match
    payment.check((client, "approved")).await
  {
    Ok(_) => println!("Payment approved"),
    Err(msg) => println!("Ocurred a error: {msg}") 
  }
}
```

## Coleta dos dados do pagamento

Para coletar os dados do pagamento, é preciso usar a função `obtain`. 
Assim, podemos coletar todas as informações necessárias para o funcionamento de seu sistema.
Você pode:

- Coletar novamente o qr_code do pagamento
- Coletar o status da transação
- Coletar a causa da falha (Caso falhar) 
- ` Entre outros dados... `

```rs
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("admin");
  let payments = &client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    amount: 22.0,
    payer_email: "test@gmail.com".into(),
    payer_cpf: "12345678909".into()
  })).await;

  if let Err(fail) = &payment_data {
    println!("Error returned when generating payment: {}", fail);
  }

  let payment = payment_data.clone().unwrap();
  let pix: &Pix = payment.access::<Pix>().unwrap();
  let collected = payments.obtain(&pix.payment_id).await.unwrap();
  println!("{:#?}", collected);
}
```