# Transaction sequencing

Attached is a mock blockchain written in Rust. This pretend blockchain
processes transactions one at a time, and supports passing transactions to
token contracts. Transactions may either read their current balance or transfer an amount.

A transaction is a signed request sent by a user. It is the job of the blockchain to validate
it as authenticate and ensure it only executes once.

# Questions

## Rust

These questions are all intended to be simple, please only use 1-2 sentences to answer.

### 1

Why is the `.clone()` method called in various places?

```
self.contract.clone()
```

### 2

Example of indexing from a map.

```
self.ledger.get(&address).map(|x| *x).unwrap_or_default()
```

Why do we do `.map(|x| *x)`?

Why do we do `.unwrap_or_default()`?

### 3

What is the purpose of using a trait like `TokenContract` ?

### 4

In the following type defintion,

```
pub struct Blockchain {
    pub block_height: u64,
    contracts: Vec<Box<dyn TokenContract>>,
    // track sequences for each address on this chain
    accounts: HashMap<String, u64>,
}
```

Why do we need to put `Box<>` in the `Vec<Box<dyn TokenContract>>` part?

### 5

What does the `?` operator do in the following?

```
self.validate_transaction_sequence(&transaction)?;
```

## Architecture / design

### 1

Why is it important to verify signatures on transactions? Note in the
code example, signature verification is omitted.

### 2

Why is it important to check the sequence number on transactions?
What would be a vulnerability if we did not do this?

Is there any problem with the current method of checking sequence numbers?

### 3

What are a couple of scaling or performance issues that might come up if millions of addresses
were using this "implementation"?

## Coding challenge

One fallback of this design is the sequence checking is done serially.

Imagine a user wants to submit many transactions in parallel. In real blockchains,
the transactions would get bundled into a block and executed together. But when this
happens, there's no guarantee on the order in which the transactions execute.

Consider the following example:

A user sends transactions numbered 1, 2, 3, 4, 5 to a blockchain in parallel.

The blockchain organizes them into a block and ends up executing them in a different order: 1, 4, 2, 3, 5.
Because of the different order, transactions 2 and 3 both cause a `BadTransactionSequence` error and don't
execute.

Let's say we cannot change the transactions from getting re-ordered due to distributed system challenges
with blockchains, but can we invent a new sequence checking mechanism that can tolerate out-of-order
transactions?

Please update the code example to illustrate your new sequence checking mechanism. It does not need
to be complete, or you can come prepared to communicate your suggestion.
