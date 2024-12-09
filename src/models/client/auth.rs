use super::product::Authorization;

impl Authorization {
  pub fn cert(certificate: impl ToString) -> Authorization {
    Authorization {
      certificate: certificate.to_string()
    }
  }
}