// Define the structure of a transaction moving amount of coins from one party (sender) to another (receiver)
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

// Define structure for a block in the blockchain
struct Block {
    timestamp: i64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

// Define structure for the blockchain
struct Blockchain {
    chain: Vec<Block>,
}


impl Blockchain {
    // Constructor for the blockchain. Initializes with a genesis block
    fn new() -> Self {
        let genesis_block = Block {
            timestamp: 0,
            transactions: vec![],
            previous_hash: String::from("0"),
            hash: String::from("0"),
            nonce: 0,
        };
        Self {
            chain: vec![genesis_block],
        }
    }
    // Method to add a new block to the blockchain
    fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        // TODO: Create the new block here with PoW and hashing
    }
}
