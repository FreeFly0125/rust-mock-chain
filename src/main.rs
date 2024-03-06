use std::collections::HashMap;
pub struct Transaction {
    // address of the sender
    pub sender: String,
    pub sequence: u64, // nonce of the sender

    // amount being sent
    pub amount: u64,

    // contract + method
    pub contract: String,
    pub method: Method,

    // destination
    pub destination: String,
}

impl Transaction {
    pub fn new(sender: &str, amount: u64, contract: &str, method: Method) -> Transaction {
        return Transaction {
            sender: sender.into(),
            amount: amount,
            contract: contract.into(),
            method: method,

            sequence: 0,
            destination: "".into(),
        };
    }
    pub fn with_seq(mut self, seq: u64) -> Transaction {
        self.sequence = seq;
        self
    }
    pub fn with_destination(mut self, destination: &str) -> Transaction {
        self.destination = destination.into();
        self
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    NotEnoughBalance,
    ContractNotFound,
    BadTransactionSequence,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Method {
    BalanceOf,
    Transfer,
}

pub trait TokenContract {
    // return the address of the token contract
    fn contract(&self) -> String;
    fn balance_of(&self, address: String) -> u64;
    fn transfer(&mut self, sender: String, amount: u64, to: String) -> Result<(), Error>;
}

pub struct BasicToken {
    contract: String,
    ledger: HashMap<String, u64>,
}

impl BasicToken {
    pub fn new(contract: String, airdrop_list: Vec<String>, initial_balance: u64) -> BasicToken {
        let mut token = BasicToken {
            contract,
            ledger: HashMap::new(),
        };

        for addr in &airdrop_list {
            // give initial balance of 1000
            token.ledger.insert(addr.clone(), initial_balance);
        }

        token
    }
}

impl TokenContract for BasicToken {
    fn contract(&self) -> String {
        // let h = Hash
        self.contract.clone()
    }
    fn balance_of(&self, address: String) -> u64 {
        self.ledger.get(&address).map(|x| *x).unwrap_or_default()
    }
    fn transfer(&mut self, sender: String, amount: u64, to: String) -> Result<(), Error> {
        println!(
            "transfer from {} to {} of {} {} amount",
            &sender, &to, amount, &self.contract
        );
        let mut balance = self.ledger.get(&sender).map(|x| *x).unwrap_or_default();
        if amount > balance {
            return Err(Error::NotEnoughBalance);
        }
        // lower balance of the source
        balance -= amount;
        self.ledger.insert(sender, balance);

        // increase balance of the destination
        let mut target_balance = self.ledger.get(&to).map(|x| *x).unwrap_or_default();
        target_balance += amount;
        self.ledger.insert(to, target_balance);

        Ok(())
    }
}

pub struct Blockchain {
    pub block_height: u64,
    contracts: Vec<Box<dyn TokenContract>>,
    // track sequences for each address on this chain
    accounts: HashMap<String, u64>,
}

impl Blockchain {
    pub fn new(contracts: Vec<Box<dyn TokenContract>>) -> Blockchain {
        Blockchain {
            block_height: 0,
            accounts: HashMap::new(),
            // instantiate two token contracts on the blockchain
            contracts: contracts,
        }
    }

    pub fn validate_transaction_sequence(
        &mut self,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        let current_sequence = self
            .accounts
            .get(&transaction.sender)
            .map(|x| *x)
            .unwrap_or_default();
        if transaction.sequence <= current_sequence {
            // invalid, the transaction sequence must increase!
            Err(Error::BadTransactionSequence)
        } else {
            // update the sequence
            self.accounts
                .insert(transaction.sender.clone(), transaction.sequence);
            Ok(())
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<u64, Error> {
        // first, validate the transaction
        // 1. validate the signature (this is important to authenticate the transaction)
        // (for brevity, this is ignored for now, but just assumed transactions are signed)

        // 2. validate the transaction is not a replay.  if we don't do this, then bad things can happen.
        self.validate_transaction_sequence(&transaction)?;

        // try to locate a contract
        for contract in &mut self.contracts {
            if contract.contract() == transaction.contract {
                return match transaction.method {
                    Method::BalanceOf => Ok(contract.balance_of(transaction.sender)),
                    Method::Transfer => contract
                        .transfer(
                            transaction.sender,
                            transaction.amount,
                            transaction.destination,
                        )
                        .map(|_| 0u64),
                };
            }
        }
        // update the "blockhash"
        self.block_height += 1;

        Err(Error::ContractNotFound)
    }
}

fn test_blockchain() -> Result<(), Error> {
    println!("This is an example blockchain.");
    let mut blockchain = Blockchain::new(vec![
        Box::new(BasicToken::new(
            "USDC".into(),
            vec!["addr1".into(), "addr2".into()],
            1000,
        )),
        Box::new(BasicToken::new(
            "WBTC".into(),
            vec!["addr3".into(), "addr4".into()],
            1000,
        )),
    ]);

    let addr1_bal = blockchain
        .process_transaction(Transaction::new("addr1", 0, "USDC", Method::BalanceOf).with_seq(1))?;

    let addr2_bal = blockchain
        .process_transaction(Transaction::new("addr2", 0, "USDC", Method::BalanceOf).with_seq(1))?;

    // initial balances of addresses are 1000
    assert!(addr1_bal == 1000);
    assert!(addr2_bal == 1000);

    // repeating a transaction is an error.
    let iserr = blockchain
        .process_transaction(Transaction::new("addr1", 0, "USDC", Method::BalanceOf).with_seq(1));
    assert!(iserr.is_err());
    assert!(iserr.err().unwrap() == Error::BadTransactionSequence);

    // test sending 100 USDC from addr1 to addr2 (increment sequence to 2)
    let _ = blockchain.process_transaction(
        Transaction::new("addr1", 100, "USDC", Method::Transfer)
            .with_seq(2)
            .with_destination("addr2"),
    )?;

    // now lookup the balances
    let addr1_bal = blockchain
        .process_transaction(Transaction::new("addr1", 0, "USDC", Method::BalanceOf).with_seq(3))?;

    let addr2_bal = blockchain
        .process_transaction(Transaction::new("addr2", 0, "USDC", Method::BalanceOf).with_seq(3))?;

    // balances changed accordingly
    assert!(addr1_bal == 900);
    assert!(addr2_bal == 1100);

    Ok(())
}

fn main() {
    let r = test_blockchain();
    if r.is_err() {
        let s = format!("{:?}", r);
        panic!("{}", s)
    }
}
