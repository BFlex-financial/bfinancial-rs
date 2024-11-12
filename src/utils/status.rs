use crate::models::server::payment::Status;

pub fn transform(api: String, complement: String) -> Status {
  match api.as_str() {
    "rejected"  => Status::Reject(complement),
    "cancelled" => Status::Cancelled,
    "approved"  => Status::Approved,
    "refunded"  => Status::Refunded,
    "pending"   => Status::Pending,
    _ => Status::Uknown
  }
}