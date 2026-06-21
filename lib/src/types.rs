use ecdsa::signature::rand_core::block;

pub struct Blockchain{
    pub blocks: Vec<Block>,
}
pub struct Block{
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
pub struct BlockHeader;
pub struct Transaction;


impl Blockchain{

    pub fn new() -> Self{
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block){
        self.blocks.push(block)
    }
}

impl Block{

    pub fn new(header: BlockHeader, transactions: Vec<Transaction>)-> Self{
        Block { header:header, transactions: transactions }
    }

    pub fn hash(&self)-> !{
        unimplemented!()
    }

}