use num::{Unsigned, Zero};
use sha2::Digest;
/// A Merkle Tree is a data structure typically used in cryptocurrency exchanges where every  "leaf" is labelled with the cryptographic hash of a data block, and every node that is not a leaf is labelled with the cryptographic hash of the labels of its child nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MerkleTree<T: Unsigned + Zero> {
    /// The nodes are represented as of `Vec<Vec<u8>>` so they can be use with the `SHA256::digest()` function
    pub nodes: Vec<Vec<T>>,
    pub levels: usize,
}

/// Which side to put Hash on when concatenating proof hashes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HashDirection {
    Left,
    Right,
}

#[derive(Debug, Default)]
/// A MerkleProof is a type of "membership proof" where a set of hashes can be used to prove that a given leaf is a member of the merkle tree
/// To prove membership of a given leaf node the two siblings are hashed to create a parent hash and then repeatedly combined with each hash in the proof, working its way up the tree  until it reaches the root node.
/// The calculated root hash can be compared with the known root hash to see if they match. Matching hashes prove that the provided leaf node is a member of the tree
pub struct Proof<'a> {
    /// The hashes to use when verifying the proof
    /// The first element of the tuple is which side the hash should be on when concatenating
    hashes: Vec<(HashDirection, &'a Vec<u8>)>,
}

#[derive(Debug)]
pub enum Error {
    CantFindDataInMerkleTree,
    IndexIsNotALeaf,
}

type Result<T> = std::result::Result<T, Error>;

impl<T> MerkleTree<T>
where
    T: Unsigned + Zero,
{
    fn construct_level_up(level: &[Vec<T>]) -> Vec<Vec<T>> {
        assert!(is_power_of_two(level.len()));

        // Step through the previous level, finding the parents by concatenating the children hashes
        level
            .chunks(2)
            .map(|pair| hash_concat(&pair[0], &pair[1]))
            .collect()
    }

    /// Constructs a Merkle tree from given input data
    pub fn construct(input: &[Vec<T>]) -> MerkleTree<T> {
        assert!(is_power_of_two(input.len()));

        // Get the hashes of our input data. These will be the leaves of the Merkle tree
        let mut hashes: Vec<Vec<Vec<T>>> = vec![input.iter().map(hash_data).collect()];
        let mut last_level = &hashes[0];

        let num_levels = (input.len() as f64).log2() as usize;

        // Iterate up the tree, one level up at a time, computing the nodes at the next level
        for _ in 0..num_levels {
            let mut next_level = vec![MerkleTree::construct_level_up(last_level)];
            hashes.append(&mut next_level);
            last_level = &hashes[hashes.len() - 1];
        }

        MerkleTree {
            nodes: hashes.into_iter().flatten().collect(),
            levels: num_levels + 1,
        }
    }

    /// Verifies that the given input data produces the given root hash
    pub fn verify(input: &[Vec<T>], root_hash: &Vec<u8>) -> bool {
        MerkleTree::construct(input).root_hash() == *root_hash
    }

    /// Returns the root hash of the Merkle tree, by returning the root node of the tree
    pub fn root_hash(&self) -> Vec<T> {
        self.nodes[self.nodes.len() - 1].clone()
    }

    /// Returns how many pieces of data were used to construct the Merkle tree
    pub fn num_leaves(&self) -> usize {
        2_usize.pow((self.levels - 1) as u32)
    }

    /// Returns the leaves (the hashes of the underlying data) of the Merkle tree
    pub fn leaves(&self) -> &[Vec<T>] {
        &self.nodes[0..self.num_leaves()]
    }

    /// Returns the index of the node that is the parent to the given node index
    fn parent_index(&self, index: usize) -> usize {
        // This function should only be used internally, so asserts here should be fine
        assert!(index != self.nodes.len() - 1, "Root node has no parent");
        assert!(index < self.nodes.len(), "Index outside of tree");

        self.nodes.len() - ((self.nodes.len() - index) / 2)
    }

    /// Produces a Merkle proof for the given leaf index
    /// returns an error if the index doesn't correspond to a leaf
    pub fn get_merkle_proof_by_index(&self, leaf_index: usize) -> Result<Proof> {
        if leaf_index >= self.num_leaves() {
            return Err(Error::IndexIsNotALeaf);
        }

        let mut proof = Proof::default();
        let mut current_known_index = leaf_index;

        for _ in 0..self.levels - 1 {
            // We already know (or already can compute) the hash of one side of
            // the pair, so just need to return the other for the proof
            let corresponding_hash = if current_known_index % 2 == 0 {
                (HashDirection::Right, &self.nodes[current_known_index + 1])
            } else {
                (HashDirection::Left, &self.nodes[current_known_index - 1])
            };

            proof.hashes.push(corresponding_hash);

            // Now we are able to calculate hash of the parent, so the parent of
            // this node is now the known node
            current_known_index = self.parent_index(current_known_index);
        }

        Ok(proof)
    }

    /// Produces a Merkle proof for the first occurrence of the given data
    /// returns an error if the data cant be found in the merkle tree
    pub fn get_merkle_proof_by_data(&self, data: &Vec<u8>) -> Result<Proof> {
        let data_hash = hash_data(data);
        let leaf_index = self
            .leaves()
            .iter()
            .position(|leaf| *leaf == data_hash)
            .ok_or(Error::CantFindDataInMerkleTree)?;

        self.get_merkle_proof_by_index(leaf_index)
    }
}

/// Verifies that the given proof is valid for a given root hash and data
pub fn verify_merkle_proof(proof: &Proof, data: &Vec<u8>, root_hash: &Vec<u8>) -> bool {
    let mut current_hash = hash_data(data);

    for (hash_direction, hash) in proof.hashes.iter() {
        current_hash = match hash_direction {
            HashDirection::Left => hash_concat(hash, &current_hash),
            HashDirection::Right => hash_concat(&current_hash, hash),
        };
    }

    current_hash == *root_hash
}
/// Compute the SHA256 digest of a given `Vec<u8>`
pub fn hash_data(data: &Vec<u8>) -> Vec<u8> {
    sha2::Sha256::digest(data).to_vec()
}
/// Take 2 sets of hashes (represented by Vec<u8>) and combine them into one vector and rehash them
pub fn hash_concat(h1: &Vec<u8>, h2: &Vec<u8>) -> Vec<u8> {
    let h3 = h1.iter().chain(h2).copied().collect();
    hash_data(&h3)
}
/// Maintain binary tree properties within the hash tree
pub fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}
