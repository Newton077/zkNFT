#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]

mod zkNft {
    use ink::{primitives::AccountId, ToAccountId};
    use ink_e2e::{sr25519::{PublicKey, Signature}, subxt::book::usage};
    use secp256k1::{All, RecoverableSignature, Message, PublicKey, RecoveryId, secp256k1};
    use ink_lang as ink;

    #[ink(storage)]
    struct NFTContract {
        delegates: ink_storage::collections::HashMap<TokenId, AccountId>,
    }

    impl NFTContract {
        #[ink(constructor)]
        fn new() -> Self {
            Self {
                delegates: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(message)]
        fn add_delegate(&mut self, tokenId: TokenId, delegate: AccountId, Signature: Vec<u8>) {
            let caller = self.env().caller();

            //verifica que la llamada sea del propietario
            assert_eq!(caller self.env().account_id(), "only the owner may include delegates");

            //delegado por id, no replicados
            assert!(
                self.delegate.get(&tokenId).is_none(),
                "A delegate for this tokenId already exists"
            );

            //verificacion de firma
            assert!(
                self.verify_owner_signature(&[tokenId, delegate], &Signature),
                "invalid signature"
            );

            //agregar delegado
            self.delegate.insert(tokenId, delegate);
        }
        // verificacion de firma
        fn verify_owner_signature(&self, data: &[u8], Signature: Vec<u8>) -> bool {
            if data.is_empty() || siganture.len() != 65 {
                return false;
            }
            let recovery_id = signature[64];
            let signature = &signature[..64];

            // veirfica la firma ECDSA
            let PublicKey = ecdsa_recover::recover_public_key(data, signature, recovery_id);
            if let Some(valid_public_key) = public_key {
                // compara la calve publica recuperada con la clave publica del propietario
                let owner_public_key: [u8; 32] = self.env().account_id().into();
                return valid_public_key == owner_public_key;
            }
            false
        }

    }
}
