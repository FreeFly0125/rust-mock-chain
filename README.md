# Transaction sequencing

This is a mock blockchain written in Rust. This pretend blockchain
processes transactions one at a time, and supports passing transactions to
token contracts. Transactions may either read their current balance or transfer an amount.

A transaction is a signed request sent by a user. It is the job of the blockchain to validate
it as authenticate and ensure it only executes once.