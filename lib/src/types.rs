use crate::U256;
use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use crate::error::{BtcError, Result};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
    pub prev_block_hash: Hash,
    pub merkle_root: MerkleRoot,
    pub target: U256,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub public_key: PublicKey,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) -> Result<()>{
        if self.blocks.is_empty(){
            if block.header.prev_block_hash != Hash::zero(){
                println!("zero Hash");
                return Err(BtcError::InvalidBlock);
            }
        }else {

            let last_block = self.blocks.last().unwrap();
            if block.header.prev_block_hash != last_block.hash(){
                println!("prev hash is wrong");
                return Err(BtcError::InvalidBlock);
            }
            if !block.header.hash().matchs_target(block.header.target){
                println!("does not match target");
                return Err(BtcError::InvalidBlock);
            }
            let merkel_root_checker = MerkleRoot::calculate(&block.transactions);
            if block.header.merkle_root != merkel_root_checker{
                println!("block merkle root is not valid!");
                return Err(BtcError::InvalidMerkleRoot);
            }

            if block.header.timestamp <= last_block.header.timestamp{
                println!("block timeatamp is not valid");
                return Err(BtcError::InvalidBlock);
            }
            block.verify_transactions(
                self.block_height(),
                &self.utxos,
                )?
            
        }
        self.blocks.push(block);
        Ok(())
    }
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header: header,
            transactions: transactions,
        }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction {
            inputs: inputs,
            outputs: outputs,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}
