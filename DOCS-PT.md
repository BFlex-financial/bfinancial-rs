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

<!-- ![Checkout page img](https://imgur.com/Y3o7FJ2.png) -->

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

  * [Criação de pagamentos](#criação-de-pagamentos)
    * [Interface checkout para compras](#interface-de-compras-checkout) 
  * **Utilitário de pagamentos** 
    * [Coletando dados do pagamento](#coletando-dados-do-pagamento) 
    * [Verificação dos status em tempo real](#validação-de-status-em-tempo-real) 

## Criação de pagamentos

Para criar um pagamento, coletamos a instância de pagamentos recebida após o
login na API via SDK, e usamos o método `create`, existente lá dentro. 

O método create, recebe um `enum`, com os campos:

```rust
pub enum PaymentCreate {
  Pix(PixCreate),
  Card(CardCreate),
  Checkout(Checkout)
}
``` 

Onde, podemos ver que o campo `Checkout(Checkout)`é especial, apenas por sua definição.
A explicação mais completa sobre o checkout, está no índice [**Interface de compras**](#interface-de-compras-checkout).

Toda vez que geramos um pagamento, recebemos um tipo `Future<Result<Response, String>>`, ou seja, preciamos
aguardar o pagamento ser validado pelo servidor da BFlex, e recebermos ou uma resposta contendo os dados do 
pagamento gerado, ou, uma mensagem de erro. O que isso quer dizer:

Sempre que usamos `payments.create(PaymentCreate::Pix(PixCreate { ... }))`, precisamos ter um `await`, para 
aguardarmos a resposta do servidor para a continuação do código; E, claro, preciamos verificar se o status é uma
resposta válida, ou uma mensagem de erro. Para validarmos se a resposta é positiva ou não, podemos usar um `if` que corta o cíclo do código, para caso dê algum problema. Exemplo:

```rust
fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(PixCreate { ... })).await;
  
  if let Err(err) = payment {
    println!("Error: {err}");
    return;
  }

  // ...
}
```

Podemos coletar também o erro, usando a keyword `match` do rust, desta forma:

```rust
match payment {
  Err(err) => {}
  Ok(data) => {}
}
```

Certo, sabemos parsear então o erro, mas e a resposta?:

A resposta pode ser parseada de formas diferentes, Caso você tenha usado o if, você pode abaixo dele simplesmente
por um `.unwrap()`, já que de uma forma ou outra, o código só passa para baixo do if caso o pagamento tiver sido
criado com sucesso. Porém, agora temos um `enum`, nomeado como `Response` em nossas mãos. Para acessarmos os dados
contidos dentro deste enumerador, podemos usar:

```rust
let data: Response = paymment.unwrap();

match data {
  Response::Pix(pix) => {},
  Response::Card(card) => {},
  Response::Checkout(checkout) => {}
}
```

Ou, se soubermos exatamente com o tipo de pagamento que estamos lidando 
(Como um trecho de código que só roda quando estamos gerando um pagamento PIX),
pode-se usar o método `access`:

```rust
let data: Reponse = payment.unwrap();
let pix = data.access::<Pix>().unwrap(); 
```

## Interface de compras (Checkout)

Como dito anteriormente, o Checkout tem algumas propriedades diferentes. 
Enquanto os outros métodos de pagamento usam `struct`s simples contendo os dados do comprador e mais algumas
informações básicas do pagamento, o Checkout tem mais complexidades, e alguns mecânismos diferentes para a realização da venda. 

A `struct` _Checkout_, contém alguns alguns campos especiais, como o campo `amount` e `products`.

- **Amount**: O campo `amount`, por mais que existente em todas as `struct`s, aqui tem um funcionamento especial.
sendo ele:

Os [Produtos catalogados](#produtos-catalogados) (Com afiliação), são sempre cobrados. Explicando melhor, se você 
quer fazer uma revenda, de um produto público ou privado, desde que não seja seu, o preço do produto tem que estar
100% sendo cobrado no `amount` do checkout, e você pode inflacionar o preço dos produtos, aumentando o `amount`, e
todo o preço sobrelacente, vai para sua Wallet BFlex. Exemplo:

- Produto 1. Limpador de parabrisas automotivo **R$100.00**
- Produto 2. Multimetro digital **R$50.00**

TODOS os produtos estão em revenda, NENHUM é seu. o valor mínimo permitido no `amount` é **R$155,00**, para que
pelo menos R$5,00 da venda sejam direcionados à sua Wallet.

ou seja, no checkout, os protudos seriam exibidos com os respectivos preços: `R$103,33` `R$51,66`  
Pois, foi inflado o valor do lucro de forma proporcional em cada um. 
Se os mesmos produtos, fossem vendidos a **R$300,00**, os exatos mesmos produtos teriam
os respectivos preços: `R$200,00` `R$100,00`. tendo você então, **100%** de
lucro em cima dos produtos.

- **Products**: O campo `products`, é um `Vec<Product>`, como o esperado. Porém, o `enum` _Product_ contém 
alguns mecânismos especiais. Sendo eles:

### Produtos customizados

Produtos customizados, são produtos 100% criados via código. Ou seja, você apenas passa uma `struct` _CustomProduct_ para dentro do `Product::Custom()`, contendo todos os dados do produto. Como: 

- `price`: Preço;
- `description`: Descrição;   
- `name`: Nome;   
- `thumbnail`: Imagem;   

Onde, os preços não necessariamente precisam bater com o valor do `amount`. Pois, se o produto está sendo criado,
quer dizer que o produto pertence à ti. Onde a resposabilidade de controle do preço final do checkout, 
é sua, como vendedor.

### Produtos catalogados

Produtos catalogados são produtos já pré-criados no painel de produtos da BFlex. Porém, na BFlex, existem produtos
públicos e produtos privados. Produtos públicos, cujo qualquer pessoa pode vender, cobrando margem de lucro sobre o preço do produto. Exemplo: 

- Um produto custa **R$1.000,00**, você pretende vendê-lo. Você deve cobrar mais que R$1.000,00 no produto, para 
que o valor do produto vá para o fornecedor, e sua margem de lucro vá para você. A margem de lucro, deve-se ser definida na diferença de valores do campo `amount` e do produto.

No exemplo a baixo, estaremos com **R$200,00** de lucro encima de algum **produto público** de R$1.000,00:

```rust
let payment: Result<Response, String> = payments.create(PaymentCreate::Checkout(Checkout {
  amount: 1200.00,
  products: vec![
    Product::Caloged(CatalogedProduct {
      product_id: "R$1.000,00 product ID".into(),
      affiliation: Affiliation::Yes(ProductType::Public)
    })
  ]
})).await;
```

> Assim como deu para ver, onde informamos para a API que o produto é público ou privado, é em caso de uma afiliação ser verdadeira. Você pode ter um exemplo de [Não afiliação, aqui](#protutos-catalogados-sem-afiliação).

Para um **produto privado**, devemos ter antes um certificado vinculado a sua conta BFlex. Onde, este certificado,
é o que configura a permissão de venda ou não. Este certificado, tem que ser emitido pelo vendedor do produto. E
de alguma forma, repassado à você.

No exemplo a baixo, estaremos com **R$200,00** de lucro encima de algum **produto privado** de R$1.000,00:

```rust
let payment: Result<Response, String> = payments.create(PaymentCreate::Checkout(Checkout {
  amount: 1200.00,
  products: vec![
    Product::Caloged(CatalogedProduct {
      product_id: "R$1.000,00 product ID".into(),
      affiliation: Affiliation::Yes(ProductType::Private(
        Authorization::cert("Certificate of permission to resell the product.")
      ))
    })
  ]
})).await;
```

### Protutos catalogados (Sem afiliação)

Os produtos sem afiliação, são produtos pré-criados de sua autoria. Onde não se precisa de nada mais do que o ID
no objeto e a definição de `affiliation` como `Affiliation::No`.

E assim como os produtos customizados, é 100% de responsabilidade sua o preço final do checkout. Não são 
obrigatóriamente cobrados, pois você sabe o preço que quer vender seu produto, nem que seja por R$0,01 centavo.

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

