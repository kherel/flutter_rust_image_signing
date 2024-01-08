use openssl::cms::{CmsContentInfo, CMSOptions};

use openssl::pkey::PKey;
use openssl::x509::X509;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn sign(certificate_contents: Vec<u8>, private_key_contents: Vec<u8>, image_contents: Vec<u8>) -> Result<Vec<u8>, String> {
  let signcert = X509::from_pem(&certificate_contents).unwrap();
  let pkey = PKey::private_key_from_pem(&private_key_contents).unwrap();
  

// Sign the image with image included in the signature
  let cms = CmsContentInfo::sign(
    Some(&signcert),
    Some(&pkey),
    None, 
    Some(&image_contents),
    CMSOptions::BINARY | CMSOptions::DETACHED
  ).unwrap();

 
  match cms.to_pem() {
    Ok(pem) => Ok(pem),
    Err(_) => Err("Failed to convert signature to PEM format".into()),
  }
}