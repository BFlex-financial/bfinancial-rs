<img align="right" src="https://imgur.com/EtCvGVc.png" height="85">

🦀 Library to assist with large and small scale payments

# BFlex Rust SDK

> [!TIP] 
> ¿Necesita ayuda con algo del SDK? Puedes intentar interactuar en [nuestra comunidad de Discord](https://discord.gg/cdEnEtwehC)

Resumen
=====================================

  <!--Tabla de índices-->
  * [Características](#características)
  * [Instalación](#instalación)
    * [Requisitos](#requisitos)
  * [Empecemos](#comencemos)
  * [Ejemplos de código](#ejemplos)

## Características

**Facilidad de implementación del SDK**: todos nuestros SDK están diseñados para mantener una estructura consistente de identificadores y patrones de uso en diferentes lenguajes de programación. Esto proporciona una integración extremadamente intuitiva: incluso si no tienes un conocimiento profundo del lenguaje específico, podrás implementar el SDK con facilidad.

**Facilidad de Obtener Resultados**: A diferencia de otras plataformas de pago, con BFlex puedes crear un pago usando tu método preferido con solo unas pocas líneas de código. Y lo mejor: todo ello de forma segura, sin necesidad de gestionar una comunicación directa con el consumidor.

**Interfaz de usuario de pago**: si su proyecto no requiere que el usuario permanezca en una plataforma específica y permite redireccionamientos, puede simplificar la implementación del lado del servidor simplemente redirigiendo al usuario a la URL de una página de pago oficial. de BFlex, asegurando una integración práctica y eficiente.

![Checkout page img](https://imgur.com/Y3o7FJ2.png)

## Instalación

### Requisitos

  * Cargo 1.7 / Rust 1.2 (o superior)

### Instalación del paquete

Para comenzar, agregue la biblioteca de BFlex a su proyecto. En el archivo `cargo.toml`, inserte la siguiente dependencia:

```toml
[dependencies]
bfinancial_rs = "*"
```

Luego, use el **[🦀 Rust](https://rust-lang.org/)** para descargar la biblioteca. Esto se puede hacer con el comando:

```sh-session
$ cargo install bfinancial_rs
```

## Empecemos

### 1. Configuración inicial

Utilice la clase **Client** del SDK para iniciar sesión con su **clave API**. Después de iniciar sesión, tendrá acceso a la instancia preconfigurada de la clase Pagos, que la clase **Client** devuelve automáticamente.

```rust
use tokio;
use bfinancial_rs::Client;

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;al payments = client.payments;
}
```

### 2. ¡Haz tu primer pago!

Pruebe la integración realizando un pago de prueba de 1 **BRL**. ¡El monto se acreditará en su cuenta **BFlex** a través de una **Pix** generada automáticamente por el SDK!

```rust
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Pix };

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into()
    amount:       1.0,
  })).await;

  match payment_data {
    Ok(pix)   => println!("{:#?}", pix.access::<Pix>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}
```

### 3. Documentación

Puedes ver la [📚 **Documentación** haciendo clic aquí](https://bflex.tech/docs/rust-sdk).

## Ejemplos

### Generando pagos con PIX
```rust
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Pix };

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into()
    amount:       1000.0,
  })).await;

  match payment_data {
    Ok(pix)   => println!("{:#?}", pix.access::<Pix>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}
```

### Generando pagos con tarjeta

```rust
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Card };

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Card(payment::CardCreate {
    expiration_month: 11,
    expiration_year:  2025,
    payer_email:     "test@gmail.com".into(),
    payer_name:      "test user".into(),
    payer_cpf:       "12345678909".into(),
    number:          "5031433215406351".into(),
    amount:           1000.0,
    cvv:             "123".into()
  })).await;


  match payment_data {
    Ok(card)  => println!("{:#?}", card.access::<Card>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}

```

### Recopilación de datos de pago

```rust
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into(),
    payer_cpf:   "12345678909".into()
    amount:       1000.0,
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

O, si no sabe el tipo exacto de pago al que se enfrenta, puede utilizar:

```rust
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::{Response, Pix}}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into(),
    payer_cpf:   "12345678909".into()
    amount:       1000.0,
  })).await;

  if let Err(fail) = &payment_data {
    println!("Error returned when generating payment: {}", fail);
  }

  let payment = payment_data.clone().unwrap();

  match payment {
    Response::Card(card) => {
      let collected = payments.obtain(&card.payment_id).await.unwrap()
      println!("{:#?}", collected);
    }

    Response::Pix(pix) => {
      let collected = payments.obtain(&pix.payment_id).await.unwrap()
      println!("{:#?}", collected);
    }
  }
  
}
```

### Validación de estado en tiempo real

Con esto podrás esperar a recibir un Estado, y saber si fue recibido, u otro.

```rust
use tokio;
use bfinancial_rs::{ models::{client::payment::{self, PaymentCreate}, server::payment::Pix}, Client};

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = &client.payments;
  let payment_data = payments.create(PaymentCreate::Pix(payment::PixCreate {
    payer_email: "test@gmail.com".into(),
    payer_cpf:   "12345678909".into()
    amount:       1000.0,
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