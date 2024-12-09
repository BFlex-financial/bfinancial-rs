<img align="right" src="https://imgur.com/EtCvGVc.png" height="85">

🦀 Library to assist with large and small scale payments

# BFlex Rust SDK

> [!TIP]
> Precisa de suporte com algo da SDK? Você pode tentar interagir em [nossa comunidade do Discord](https://discord.gg/cdEnEtwehC)

Sumário
=========================================

  <!--Tabela de indices-->
  * [Funcionalidades](#funcionalides)
  * [Instalação](#instalação)
    * [Requisitos](#requisitos)
  * [Vamos começar](#vamos-começar)
  * [Exemplos de código](#exemplos)

## Funcionalidades

**Facilidade de Implementação das SDKs**: Todas as nossas SDKs são projetadas para manter uma estrutura consistente de identificadores e modos de uso em diferentes linguagens de programação. Isso proporciona uma integração extremamente intuitiva: mesmo que você não tenha um conhecimento profundo da linguagem específica, será capaz de implementar a SDK com facilidade.

**Facilidade na Obtenção de Resultados**: Diferentemente de outras plataformas de pagamento, na BFlex você pode, com apenas algumas linhas de código, criar um pagamento utilizando o método de sua preferência. E o melhor: tudo isso de forma segura, sem a necessidade de gerenciar a comunicação direta com o consumidor.

**Interface de Usuário para Checkout**: Caso o seu projeto não exija que o usuário permaneça em uma plataforma específica e permita redirecionamentos, você pode simplificar a implementação no lado do servidor. Basta redirecionar o usuário para uma URL oficial da página de checkout da BFlex, garantindo uma integração prática e eficiente.

![Checkout page img](https://imgur.com/Y3o7FJ2.png)
<div align="center">

###### Imagem ilustrativa temporária
</div>
## Instalação

### Requisitos

  * Cargo 1.7 / Rust 1.2 (ou superior)

### Instalação do pacote

Para começar, adicione a biblioteca da BFlex ao seu projeto. No arquivo `cargo.toml`, insira a seguinte dependência:

```toml
[dependencies]
bfinancial_rs = "*"
```

Depois, utilize a SDK do **[🦀 Rust](https://rust-lang.org/)** para baixar a biblioteca. Isso pode ser feito com o comando:

```sh-session
$ cargo install bfinancial_rs
```

## Vamos começar


### 1. Configuração incial

Utilize a classe **Client** da SDK para realizar o login com sua **chave de API**. Após o login, você terá acesso à instância pré-configurada da classe Payments, que é retornada automaticamente pela classe **Client**.

```rust
use tokio;
use bfinancial_rs::Client;

#[tokio::main]
async fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;al payments = client.payments;
}
```

### 2. Realize seu primeiro pagamento!

Experimente a integração realizando um pagamento de teste no valor de 1 **BRL**. O montante será creditado em sua conta **BFlex** por meio de um **Pix** gerado automaticamente pela SDK!

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

### 3. Documentação

Você pode ver a [📚 **Documentação** apertando aqui](https://bflex.tech/docs/rust-sdk). 

## Exemplos

### Gerando pagamentos com PIX

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

### Gerando pagamentos com Cartão

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

### Coletando dados do pagamento

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

Ou, caso você não saiba o tipo exato de pagamento com que está lidando, você pode usar:

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

### Validação de Status em tempo real

Com isto, você pode aguardar o recebimento de um Status, e saber se foi recebido ele, ou outro.

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

