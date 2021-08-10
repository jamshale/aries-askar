#[cfg(feature = "getrandom")]
#[test]
fn sign_expected() {
    use askar_bbs::{DynGeneratorsV1, Message, SignatureMessages};
    use askar_crypto::{
        alg::bls::{BlsKeyPair, G2},
        repr::KeySecretBytes,
    };
    use hex_literal::hex;

    let keypair = BlsKeyPair::<G2>::from_secret_bytes(&hex!(
        "0011223344556677889900112233445566778899001122334455667788990011"
    ))
    .unwrap();
    let messages = [Message::hash("hello")];
    let gens = DynGeneratorsV1::new(&keypair, messages.len());
    let mut builder = SignatureMessages::new(&gens);
    builder
        .append(messages.iter().copied())
        .expect("Error building signature");
    let sig = builder.sign(&keypair).expect("Error creating signature");
    let verify = builder.verify_signature(&keypair, &sig).unwrap();
    assert!(verify);
}
