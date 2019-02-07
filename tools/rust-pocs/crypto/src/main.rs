// Extracted from (with some modifications):
// https://github.com/RustCrypto/block-ciphers

#[macro_use] extern crate hex_literal;

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use aes_soft::Aes128;

// create an alias for convinience
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
  let key = hex!("000102030405060708090a0b0c0d0e0f");
  let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
  let plaintext = b"Hello world!";
  let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();

  // buffer must have enough space for message+padding
  let mut buffer: Vec<u8> = plaintext.to_vec();
  for _ in 0..1000 {
    buffer.push(0u8);
  }
  // copy message to the buffer
  let pos = plaintext.len();
  buffer[..pos].copy_from_slice(plaintext);
  let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

  assert_eq!(ciphertext, hex!("1b7a4c403124ae2fb52bedc534d82fa8"));

  // re-create cipher mode instance and decrypt the message
  let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
  let mut buf = ciphertext.to_vec();
  let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();

  assert_eq!(decrypted_ciphertext, plaintext);

  println!("decrypted_ciphertext {:?}", std::str::from_utf8(decrypted_ciphertext).unwrap());
}
