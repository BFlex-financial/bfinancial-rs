#[cfg(test)]
mod lib_tests {
  #[test]
  fn test_spawn() {
    println!("{:#?}", bfinancial_rs::models::client::payment::PixCreate { amount: 2.0, payer_email: "lucasdwbfff@gmail.com".into() });
  }
}