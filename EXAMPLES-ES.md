<div align="center">
  <h1>Ejemplos de código</h1>
</div>

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

### Tarjeta

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

## Verificación del estado del pago

La verificación del estado funciona de la siguiente manera: Si está verificado como PENDIENTE, el pago continuará verificando hasta que haya algún cambio en el estado. Cuando el estado cambie a cualquier otro, recibiremos algún tipo de respuesta.

Si cambia al estado esperado por la VERIFICACIÓN, recibirás un Ok de la SDK.

Si cambia a cualquier otro estado, NO al esperado, recibirás un error en Err.

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
    Ok(_) => println!("Pago aprobado"),
    Err(msg) => println!("Ocurrió un error: {msg}") 
  }
}
```

## Recopilación de los datos del pago

Para recopilar los datos del pago, es necesario usar la función `obtain`. Así, podemos recopilar toda la información necesaria para el funcionamiento de tu sistema. Puedes:

- Recopilar el código QR del pago nuevamente
- Recopilar el estado de la transacción
- Recopilar la causa del fallo (si falló)
- `Entre otros datos...`

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