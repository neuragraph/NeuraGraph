use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(data);
    hasher.result_str()
}
