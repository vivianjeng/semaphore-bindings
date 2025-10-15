use std::sync::Arc;

use semaphore::proof::{GroupOrMerkleProof, Proof, SemaphoreProof};

use crate::{group::Group, identity::Identity};

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ProofError {
    #[error("Invalid group")]
    InvalidGroup,

    #[error("Invalid proof")]
    InvalidProof,

    #[error("Invalid proof export")]
    InvalidProofExport,

    #[error("Invalid proof import")]
    InvalidProofImport,
}

#[uniffi::export]
pub fn generate_semaphore_proof(
    identity: Arc<Identity>,
    group: Arc<Group>,
    message: String,
    scope: String,
    merkle_tree_depth: u16,
) -> Result<String, ProofError> {
    let semaphore_identity = semaphore::identity::Identity::new(&identity.private_key());
    let semaphore_group = semaphore::group::Group::new(
        &group
            .members()
            .iter()
            .map(|m| m.to_vec().try_into().unwrap())
            .collect::<Vec<[u8; 32]>>(),
    )
    .map_err(|_| ProofError::InvalidGroup)?;
    let semaphore_proof = Proof::generate_proof(
        semaphore_identity,
        GroupOrMerkleProof::Group(semaphore_group),
        message,
        scope,
        merkle_tree_depth,
    )
    .map_err(|_| ProofError::InvalidProof)?;
    Ok(semaphore_proof
        .export()
        .map_err(|_| ProofError::InvalidProofExport)?)
}

#[uniffi::export]
pub fn verify_semaphore_proof(proof: String) -> Result<bool, ProofError> {
    let semaphore_proof =
        SemaphoreProof::import(&proof).map_err(|_| ProofError::InvalidProofImport)?;
    Ok(Proof::verify_proof(semaphore_proof))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_semaphore_proof() {
        let identity = Identity::new("secret".as_bytes().to_vec());
        let group = Group::new(vec![identity.to_element()]);
        let message = "test";
        let scope = "test";
        let merkle_tree_depth = 16;
        let proof = generate_semaphore_proof(
            Arc::new(identity),
            Arc::new(group),
            message.to_string(),
            scope.to_string(),
            merkle_tree_depth,
        );
        let proof = proof.unwrap();
        let verified = verify_semaphore_proof(proof);
        assert!(verified.is_ok());
        assert!(verified.unwrap());
    }
}
