use ed25519_dalek::VerifyingKey;
use libp2p::{PeerId, multihash::Multihash};

use super::Error;

/// Extracts an Ed25519 VerifyingKey directly from a PeerId.
pub fn try_verifying_key_from_peer_id(peer_id: PeerId) -> Result<VerifyingKey, Error> {
    let multihash = Multihash::from(peer_id);
    // Code 0x00 indicates the 'identity' multihash (used for inlining keys)
    if multihash.code() != 0x00 {
        return Err(Error::NoInlinedKey);
    }
    // Decode
    let pk = libp2p::identity::PublicKey::try_decode_protobuf(multihash.digest())?;
    // Downcast and coerce
    pk.try_into_ed25519()
        .map(|inner| libp2p_to_dalek(&inner))
        .map_err(|_| Error::NotEd25519)
}

pub fn dalek_to_libp2p(value: &VerifyingKey) -> libp2p::identity::ed25519::PublicKey {
    libp2p::identity::ed25519::PublicKey::try_from_bytes(&value.to_bytes()).unwrap()
}

pub fn libp2p_to_dalek(value: &libp2p::identity::ed25519::PublicKey) -> VerifyingKey {
    VerifyingKey::from_bytes(&value.to_bytes()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use libp2p::identity::Keypair;
    use rand::Rng;

    fn gen_seed() -> [u8; 32] {
        let mut seed = [0u8; 32];
        rand::rng().fill(&mut seed);
        seed
    }

    fn gen_keys() -> ([u8; 32], SigningKey, VerifyingKey) {
        let seed = gen_seed();
        let sk = SigningKey::from_bytes(&seed);
        let vk = sk.verifying_key();
        (seed, sk, vk)
    }

    #[test]
    fn test_100_verifying_key_roundtrips() {
        for _ in 0..100 {
            let (seed, _, vk) = gen_keys();
            let lib_pk = dalek_to_libp2p(&vk);
            let pid = PeerId::from_public_key(&lib_pk.into());
            let recovered = try_verifying_key_from_peer_id(pid)
                .unwrap_or_else(|e| panic!("Fail on seed {:?}: {}", seed, e));
            assert_eq!(vk, recovered, "Mismatch on seed {:?}", seed);
        }
    }

    #[test]
    fn test_100_peer_id_roundtrips() {
        for _ in 0..100 {
            let (seed, sk, _) = gen_keys();
            let lib_kp = Keypair::ed25519_from_bytes(sk.to_bytes()).unwrap();
            let pid = PeerId::from_public_key(&lib_kp.public());
            let vk = try_verifying_key_from_peer_id(pid)
                .unwrap_or_else(|e| panic!("Fail on PeerId {}: {}", pid, e));
            let lib_pk = dalek_to_libp2p(&vk);
            let final_pid = PeerId::from_public_key(&lib_pk.into());
            assert_eq!(pid, final_pid, "Mismatch on seed {:?}", seed);
        }
    }
}
