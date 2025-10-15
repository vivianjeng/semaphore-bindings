use anyhow::Result;
use lean_imt::hashed_tree::HashedLeanIMT;
use semaphore::group::PoseidonHash;
use semaphore::group::ELEMENT_SIZE;
use semaphore::group::EMPTY_ELEMENT;
use uniffi::Object;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum GroupError {
    #[error("Empty leaf is not allowed")]
    EmptyLeaf,

    #[error("Member must be exactly 32 bytes")]
    InvalidMemberLength,

    #[error("Removed member is not allowed")]
    RemovedMember,

    #[error("Already removed member is not allowed")]
    AlreadyRemovedMember,
}

#[derive(Debug, Clone, Object)]
pub struct Group {
    tree: HashedLeanIMT<ELEMENT_SIZE, PoseidonHash>,
}

#[uniffi::export]
impl Group {
    #[uniffi::constructor]
    pub fn new(members: Vec<Vec<u8>>) -> Self {
        let vec_members: Vec<[u8; 32]> = members
            .iter()
            .map(|v| {
                v.as_slice()
                    .try_into()
                    .expect("each Vec<u8> must be exactly 32 bytes")
            })
            .collect();
        let group = semaphore::group::Group::new(&vec_members).unwrap();
        Self { tree: group.tree }
    }

    pub fn root(&self) -> Option<Vec<u8>> {
        self.tree.root().map(|r| r.to_vec())
    }

    pub fn depth(&self) -> u32 {
        self.tree.depth() as u32
    }

    pub fn members(&self) -> Vec<Vec<u8>> {
        self.tree.leaves().iter().map(|l| l.to_vec()).collect()
    }

    pub fn index_of(&self, member: Vec<u8>) -> Option<u32> {
        self.tree.index_of(&member).map(|i| i as u32)
    }

    pub fn add_member(&self, member: Vec<u8>) -> Result<(), GroupError> {
        if member == EMPTY_ELEMENT {
            return Err(GroupError::EmptyLeaf);
        }

        let member_array: [u8; 32] = member
            .as_slice()
            .try_into()
            .expect("each Vec<u8> must be exactly 32 bytes");
        let mut new_tree = self.tree.clone();
        new_tree.insert(&member_array);
        Ok(())
    }

    pub fn add_members(&self, members: Vec<Vec<u8>>) -> Result<(), GroupError> {
        let vec_members: Vec<[u8; 32]> = members
            .iter()
            .map(|v| {
                v.as_slice()
                    .try_into()
                    .expect("each Vec<u8> must be exactly 32 bytes")
            })
            .collect();
        for &member in &vec_members {
            if member == EMPTY_ELEMENT {
                return Err(GroupError::EmptyLeaf);
            }
        }

        let mut new_tree = self.tree.clone();
        new_tree
            .insert_many(&vec_members)
            .map_err(|_| GroupError::EmptyLeaf)?;
        Ok(())
    }

    pub fn update_member(&self, index: u32, member: Vec<u8>) -> Result<(), GroupError> {
        if self.members()[index as usize] == EMPTY_ELEMENT {
            return Err(GroupError::EmptyLeaf);
        }

        let member_array: [u8; 32] = member
            .as_slice()
            .try_into()
            .expect("each Vec<u8> must be exactly 32 bytes");
        let mut new_tree = self.tree.clone();
        new_tree
            .update(index as usize, &member_array)
            .map_err(|_| GroupError::EmptyLeaf)?;
        Ok(())
    }

    pub fn remove_member(&self, index: u32) -> Result<(), GroupError> {
        if self.members()[index as usize] == EMPTY_ELEMENT {
            return Err(GroupError::EmptyLeaf);
        }

        let mut new_tree = self.tree.clone();
        new_tree
            .update(index as usize, &EMPTY_ELEMENT)
            .map_err(|_| GroupError::EmptyLeaf)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::identity::Identity;

    use super::*;

    #[test]
    fn test_group() {
        let identity = Identity::new("secret".as_bytes().to_vec());
        let identity2 = Identity::new("secret2".as_bytes().to_vec());
        let group = Group::new(vec![identity.to_element(), identity2.to_element()]);
        println!("{:?}", group.root());
    }
}
