extern crate sha2;
use sha2::{Sha256, Digest};
// Define the structure of a transaction moving amount of coins from one party (sender) to another (receiver)
#[derive(Clone)]
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
    pending_transactions: Vec<Transaction>,

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
            pending_transactions: vec![],

        }
    }
    // Method to add a new block to the blockchain
    fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        // TODO: Create the new block here with PoW and hashing
    }
}

// Placeholder for the hashing method on the Block
impl Block {
        fn calculate_hash(&self) -> String {
        let transactions_as_string = self.transactions.iter()
            .map(|tx| format!("{}->{}:{}", tx.sender, tx.receiver, tx.amount))
            .collect::<Vec<String>>()
            .join(",");
            
        let contents = format!("{}{}{}{}", self.timestamp, transactions_as_string, self.previous_hash, self.nonce);
        
        let mut hasher = Sha256::new();
        hasher.update(contents);
        format!("{:x}", hasher.finalize())
    }
}

// Placeholder for the Proof-of-Work mechanism on the Blockchain
impl Blockchain {
    // Method implementing the PoW mechanism
    fn proof_of_work(&self, block: &mut Block) {
        // TODO: Implement PoW by modifying block's nonce until the hash has the required number of leading zeroes
    }
    // Method to add a new transaction to the next block to be mined.
    fn add_transaction(&mut self, sender: String, receiver: String, amount: f32) {
        let transaction = Transaction { sender, receiver, amount };
        self.pending_transactions.push(transaction);
    }
    // Mine a new block from pending transactions
    fn mine_block(&mut self) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block {
            timestamp: 0, // You can use the current timestamp here
            transactions: self.pending_transactions.clone(),
            previous_hash,
            hash: String::from("0"),
            nonce: 0,
        };
        self.proof_of_work(&mut new_block);
        self.chain.push(new_block);
        self.pending_transactions.clear();
    }
}

#[derive(Clone)]
struct SmartContract {
        sender: String,
        receiver: String,
        amount: f32,
        condition_block_height: usize,
}


impl Blockchain {
    // Method to execute a given smart contract and create transactions based on its conditions.
    fn execute_contract(&mut self, contract: SmartContract) {
        if self.chain.len() >= contract.condition_block_height {
            // Execute the contract by adding a transaction
            self.add_transaction(contract.sender, contract.receiver, contract.amount);
        }
    }
    // Method to verify the integrity of the blockchain.
    fn verify_chain(&self) -> bool {
        // TODO: Check the chain for any discrepancies or tampering.
        true
    }
}


fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();
    println!("Created a new blockchain.");

    // Add some transactions
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 50.0);
    blockchain.add_transaction("Bob".to_string(), "Charlie".to_string(), 25.0);
    println!("Added transactions to the blockchain.");

    // Mine a new block
    blockchain.mine_block();
    println!("Mined a new block.");

    // Add a smart contract
    let contract = SmartContract {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 100.0,
        condition_block_height: 3,
    };

    // Add more transactions and mine blocks while checking the smart contract
    for _ in 0..3 {
        blockchain.add_transaction("Charlie".to_string(), "Alice".to_string(), 10.0);
        blockchain.mine_block();
        blockchain.execute_contract(contract.clone());
        println!("Mined another block and checked smart contract execution.");
    }

    // Print the blocks
    for block in &blockchain.chain {
        println!("Block:");
        println!("Timestamp: {}", block.timestamp);
        for tx in &block.transactions {
            println!("Transaction: {} -> {}: {}", tx.sender, tx.receiver, tx.amount);
        }
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}", block.nonce);
        println!("-----");
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_blockchain() {
//         let blockchain = Blockchain::new();
//         assert_eq!(blockchain.chain.len(), 1); // Only the genesis block should be present
//     }

//     #[test]
//     fn test_add_transaction() {
//         let mut blockchain = Blockchain::new();
//         blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 50.0);
//         assert_eq!(blockchain.pending_transactions.len(), 1);
//     }

//     #[test]
//     fn test_mine_block() {
//         let mut blockchain = Blockchain::new();
//         blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 50.0);
//         blockchain.mine_block();
//         assert_eq!(blockchain.chain.len(), 2); // Genesis block + the mined block
//     }

//     #[test]
//     fn test_execute_contract() {
//         let mut blockchain = Blockchain::new();
//         let contract = SmartContract {
//             sender: "Alice".to_string(),
//             receiver: "Bob".to_string(),
//             amount: 100.0,
//             condition_block_height: 2,
//         };
//         blockchain.mine_block(); // This will make the blockchain length 2
//         blockchain.execute_contract(contract);
//         assert_eq!(blockchain.pending_transactions.len(), 1); // Contract should have been executed
//     }
// }
