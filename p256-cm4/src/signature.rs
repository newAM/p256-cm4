use p256::U32;
use signature::digest::{OutputSizeUser, core_api::EagerHash};

use crate::{Signature, VerifyingKey};

impl<D> signature::DigestVerifier<D, crate::Signature> for VerifyingKey
where
    D: signature::digest::Update + EagerHash + OutputSizeUser<OutputSize = U32>,
{
    fn verify_digest<F>(&self, f: F, signature: &Signature) -> Result<(), signature::Error>
    where
        F: Fn(&mut D) -> Result<(), signature::Error>,
    {
        let mut digest = D::new();
        f(&mut digest)?;
        let digest = digest.finalize();
        self.verify_prehash(digest.as_ref(), signature)
            .then_some(())
            .ok_or_else(signature::Error::new)
    }
}

#[cfg(feature = "p256")]
impl<D> signature::DigestVerifier<D, p256::ecdsa::Signature> for VerifyingKey
where
    D: signature::digest::Update + EagerHash + OutputSizeUser<OutputSize = U32>,
{
    fn verify_digest<F>(
        &self,
        f: F,
        signature: &p256::ecdsa::Signature,
    ) -> Result<(), signature::Error>
    where
        F: Fn(&mut D) -> Result<(), signature::Error>,
    {
        let signature = signature.into();

        let mut digest = D::new();
        f(&mut digest)?;
        let digest = digest.finalize();
        self.verify_prehash(digest.as_ref(), &signature)
            .then_some(())
            .ok_or_else(signature::Error::new)
    }
}
