<img align="right" src="https://imgur.com/EtCvGVc.png" height="85">

🦀 Library to assist with large and small scale payments

# BFlex Rust SDK

> [!TIP]
> ¿Necesitas soporte con algo del SDK? Puedes intentar interactuar en [nuestra comunidad de Discord](https://discord.gg/cdEnEtwehC)

Resumen
=========================================

  <!--Tabela de indices-->
  * [Características](#características)
  * [Instalación](#instalación)
    * [Requisitos](#requisitos)
  * [Vamos a empezar](#Empecemos)
  * [Ejemplos de código](#ejemplos)

## Características

**Facilidad de implementación del SDK**: todos nuestros SDK están diseñados para mantener una estructura consistente de identificadores y modos de uso en diferentes lenguajes de programación. Esto proporciona una integración extremadamente intuitiva: incluso si no tienes un conocimiento profundo del lenguaje específico, podrás implementar el SDK con facilidad.

**Resultados fáciles de obtener**: A diferencia de otras plataformas de pago, en BFlex puedes, con solo unas pocas líneas de código, crear un pago utilizando el método de tu elección. Y lo mejor: todo ello de forma segura, sin necesidad de gestionar la comunicación directa con el consumidor.

**Interfaz de usuario de pago**: si su proyecto no requiere que el usuario permanezca en una plataforma específica y permite redireccionamientos, puede simplificar la implementación en el lado del servidor. Simplemente redirija al usuario a la URL de la página de pago oficial de BFlex, lo que garantiza una integración práctica y eficiente.

<!-- ![Checkout page img](https://imgur.com/Y3o7FJ2.png) -->

## Instalación

### Requisitos

  * Cargo 1.7 / Rust 1.2 (tú superior)

### Instalación del paquete

Para comenzar, agregue la biblioteca BFlex a su proyecto. En el archivo `cargo.toml`, inserte la siguiente dependencia:

```toml
[dependencies]
bfinancial_rs = "*"
```

Luego, use el SDK **[🦀 Rust](https://rust-lang.org/)** para descargar la biblioteca. Esto se puede hacer con el comando:

```sh-session
$ cargo install bfinancial_rs
```

## Empecemos


### 1. Configuración inicial

Utilice la clase **Cliente** del SDK para iniciar sesión con su **clave API**. Después de iniciar sesión, tendrá acceso a la instancia preconfigurada de la clase Pagos, que la clase **Cliente** devuelve automáticamente.

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

Pruebe la integración realizando un pago de prueba de 1 **ARS**. ¡El monto se acreditará en su cuenta **BFlex** a través de una **Card** generada automáticamente por el SDK!

```rust
use tokio;
use bfinancial_rs::{ Client, models::client::payment::{self, PaymentCreate}, models::server::payment::Pix };

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
    Ok(pix)   => println!("{:#?}", pix.access::<Pix>().unwrap()),
    Err(fail) => println!("Error returned when generating payment: {}", fail)
  }
}
```

### 3. Documentación

Puedes ver la [📚 **Documentación** haciendo clic aquí](https://bflex.tech/docs/rust-sdk).

## Ejemplos

  * [Creando pagos](#creando-pagos)
    * [Interfaz de pago para compras](#interfaz-de-pago-para-compras-checkout) 
  * **Utilidades de pago** 
    * [Recopilación de datos de pago](#recopilación-de-datos-de-pago) 
    * [Validación de estado en tiempo real](#validación-de-estado-en-tiempo-real) 

## Creando pagos

Para crear un pago, recopilamos la instancia de pagos recibida después de la
inicie sesión en la API a través del SDK y usamos el método `create`, que existe dentro.

El método de creación recibe una `enum`, con los campos:

```rust
pub enum PaymentCreate {
  Pix(PixCreate),
  Card(CardCreate),
  Checkout(Checkout)
}
``` 

Donde podemos ver que el campo `Checkout(Checkout)` es especial, solo por su definición.
La explicación más completa sobre el pago se encuentra en el índice [**Interfaz de pago para compras**](#interfaz-de-pago-para-compras-checkout).

Cada vez que generamos un pago, recibimos un tipo `Future<Result<Response, String>>`, es decir, necesitamos
esperar a que el pago sea validado por el servidor BFlex y recibiremos una respuesta que contiene los detalles del pago
pago generado, o, un mensaje de error. Qué quiere decir esto:

Siempre que usamos `payments.create(PaymentCreate::Pix(PixCreate { ... }))`, necesitamos tener un `await`, para
esperamos la respuesta del servidor para continuar con el código; Y, por supuesto, debemos comprobar si el estado es un
respuesta válida o un mensaje de error. Para validar si la respuesta es positiva o no, podemos usar un `if` que corta el ciclo del código, en caso de que haya algún problema. Ejemplo:

```rust
fn main() {
  let client = Client::login("YOUR_API_KEY");
  let payments = client.payments;
  let payment: Result<Response, String> = payments.create(PaymentCreate::Pix(PixCreate { ... })).await;
  
  if let Err(err) = &payment {
    println!("Error: {err}");
    return;
  }

  // ...
}
```

También podemos recopilar el error utilizando la palabra clave Rust `match`, de esta manera:

```rust
match payment {
  Err(err) => {}
  Ok(data) => {}
}
```

Bien, sabemos cómo analizar el error, pero ¿qué pasa con la respuesta?:

La respuesta se puede analizar de diferentes maneras. Si usó if, simplemente puede hacerlo.
por `.unwrap()`, ya que de una forma u otra el código solo pasa por debajo del if si el pago ha sido
creado exitosamente. Sin embargo, ahora tenemos una `enum`, llamada `Response` en nuestras manos. Para acceder a los datos
contenido dentro de este enumerador, podemos usar:

```rust
let data: Response = paymment.unwrap();

match data {
  Response::Pix(pix) => {},
  Response::Card(card) => {},
  Response::Checkout(checkout) => {}
}
```

O, si sabemos exactamente a qué tipo de pago nos enfrentamos
(Como un fragmento de código que solo se ejecuta cuando generamos un pago PIX),
puedes usar el método `access`:

> [!TIP]
> Acceso, en un solo pago `PIX`, y en un solo pago las `Card` son `struct´s. Mientras el proceso de pago es de tipo `String`

```rust
let data: Reponse = payment.unwrap();
let pix = data.access::<Pix>().unwrap(); 
```

## Interfaz de pago para compras (Checkout)

Como se indicó anteriormente, Checkout tiene algunas propiedades diferentes.
Mientras que otros métodos de pago utilizan `struct` simples que contienen los datos del comprador y algunos más
información básica de pago, Checkout tiene más complejidades y algunos mecanismos diferentes para realizar la venta.

La `struct` _Checkout_ contiene algunos campos especiales, como los campos `amount` y `products`.

- **Cantidad**: El campo `amount`, aunque existe en todas las `struct`s, tiene una función especial aquí.
ser:

[Productos catalogados](#productos-catalogados) (Con afiliación) siempre se cobran. Explicando mejor, si
Si desea revender un producto público o privado, siempre que no sea suyo, el precio del producto debe ser
Se cobra el 100% al finalizar la compra "monto", y puede inflar el precio de los productos, aumentando el "monto", y
todo el exceso de precio va a su BFlex Wallet. Ejemplo:

- Producto 1. Limpiador de parabrisas para automóviles **$100,00**
- Producto 2. Multímetro digital **$50,00**

TODOS los productos son para reventa, NINGUNO es tuyo. el valor mínimo permitido en `amount` es **$155,00**, de modo que
Se enviarán al menos $5,00 de la venta a su Wallet.

es decir, al momento de realizar el pago, los productos serían mostrados con sus respectivos precios: `$103,33` `$51,66`

Por lo tanto, el valor de la ganancia se infló proporcionalmente en cada uno.

Si los mismos productos se vendieran por **R$ 300,00**, exactamente los mismos productos tendrían
los precios respectivos: `$200,00` `$100,00`. tenerte entonces, **100%** de
ganancias sobre los productos.

- **Productos**: El campo `productos` es un `Vec<Product>`, como se esperaba. Sin embargo, el _Product_ `enum` contiene
algunos mecanismos especiales. Ellos son:

### Productos customizados

Los productos personalizados son productos 100% creados mediante código. En otras palabras, simplemente pasa una `struct` _CustomProduct_ a `Product::Custom()`, que contiene todos los datos del producto. Como:

- `price`: Precio;
- `description`: Descripción;   
- `name`: Nombre;   
- `thumbnail`: Imagen;   

Donde los precios no necesariamente tienen que coincidir con el valor de la "cantidad". Porque, si el producto se está creando,
Esto significa que el producto te pertenece. Donde la responsabilidad de controlar el precio final de pago,
Es tuyo, como vendedor.

### Productos catalogados

Los productos catalogados son productos ya creados previamente en el panel de productos BFlex. Sin embargo, en BFlex hay productos
productos públicos y privados. Productos públicos, que cualquiera puede vender, cobrando un margen de beneficio sobre el precio del producto. Ejemplo:

- Un producto cuesta `$1.000,00`, usted pretende venderlo. Debe cobrar más de R$ 1.000,00 por el producto, para
que el valor del producto va al proveedor y su margen de beneficio va a usted. El margen de beneficio debe definirse como la diferencia entre los valores del campo `monto` y el producto.

En el siguiente ejemplo, tendremos $200,00 de ganancia además de algún **producto público** de `$1000,00`:

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

> Como puedes ver, donde informamos a la API que el producto es público o privado es si una afiliación es verdadera. Puedes tener un ejemplo de [No afiliación, aquí](#productos-catalogados-sin-afiliación).

Para un **producto privado**, primero debemos tener un certificado vinculado a su cuenta BFlex. Donde, este certificado,
Es lo que determina el permiso de venta o no. Este certificado debe ser emitido por el vendedor del producto. Y
de alguna manera te lo transmitieron.

En el siguiente ejemplo, tendremos **$200,00** de ganancia además de algún **producto privado** de $1.000,00:

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

### Productos catalogados (Sin afiliación)

Los productos sin afiliación son productos precreados de su propia creación. Donde no necesitas nada más que el ID
en el objeto y estableciendo `affiliation`  en `Affiliation::No`.

Y al igual que los productos personalizados, el precio final al finalizar la compra es 100% tu responsabilidad. ellos no son
necesariamente cobrado, porque usted sabe el precio que desea vender su producto, incluso si es de $0,01 centavo.

### Recopilación de datos de pago

Los datos de pago están protegidos y solo se puede acceder a ellos con la clave API del creador del pago.
Las posibles respuestas de error ante fallas en el cobro de pagos son:

- `Payment not found • 404`: Pago por ID `x`
(Informado como primer argumento en el método `obtain`)
no existe o simplemente no fue encontrado.

- `Unauthorized access • 401`: ID de pago `x`
(Informado como primer argumento en el método `obtain`)
fue encontrado, pero el autor de la creación del pago
no eres tú.

En el siguiente ejemplo, se supone que tiene un pago en formato "Card" que ya ha sido creado.
y usted, por algún motivo, desea recopilar los datos de pago,
tales como: Estado; Información de intereses;

```rust
let pix: &Pix = payment.access::<Pix>().unwrap();
let collected = payments.obtain(&pix.payment_id).await.unwrap();
```

### Validación de estado en tiempo real

Con el método `check` puedes consultar el estado de un pago, comprobar si está pendiente, si ha sido pagado…
Sin embargo, en caso de éxito, la respuesta se presenta como una `Option` invertida. Lo nombramos aquí: `Verified<String>`

> [!TIP]
> import _Verified_ `Verified::{self, *}`

```rust
match
  payment.check((client, "approved")).await
{
  Sucess => println!("Payment approved"),
  Fail(msg) => println!("Ocurred a error: {msg}") 
}
```

Si no desea esperar a que se produzcan cambios en el pago, puede abrir un hilo y utilizar el
revisa en un segundo hilo.

Puede hacer esto usando la biblioteca de subprocesos estándar de Rust y usar mpsc para abrir un canal.
comunicación entre el hilo y el código principal (Consejo, pero opcional). Entonces, como en este ejemplo:

```rust
thread::spawn(move || {
  match
    payment.check((client, "approved")).await
  {
    Sucess => println!("Payment approved"),
    Fail(msg) => println!("Ocurred a error: {msg}")
  }
});
```