use serde::{ Serialize, Deserialize };

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Product {
  Custom(CustomProduct),
  Cataloged(CatalogedProduct)
} 

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CatalogedProduct {
  pub product_id: String,
  pub affiliation: Affiliation
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "answer", content = "info")]
pub enum Affiliation {
  Yes(ProductType),
  No
} 

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ProductType {
  Private(Authorization),
  Public
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Authorization {
  pub(crate) certificate: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CustomProduct {
  pub price: f64,
  pub thumbnail: String,
  pub name: String,
  pub description: String
}