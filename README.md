# ExpenseTracker

## Project Description

ExpenseTracker is a Stellar Soroban smart contract designed to record simple income and expense transactions.

This project allows users to store transaction data such as title, amount, category, and transaction type. The transaction type can be either `income` for money received or `expense` for money spent.

With ExpenseTracker, users can view all transactions, calculate total income, calculate total expenses, and check the final balance based on the stored transaction records.

## Project Vision

The vision of ExpenseTracker is to help users manage and monitor their personal finances in a simple, transparent, and decentralized way using blockchain technology.

This project can also become a foundation for more advanced financial applications in the future, such as budgeting tools, financial reports, transaction history tracking, and decentralized personal finance management.

## Project Features

ExpenseTracker provides several main features:

1. Add a new transaction

   Users can create a new transaction by entering the title, amount, category, and transaction type.

2. View all transactions

   Users can view all transactions stored in the smart contract.

3. Update a transaction

   Users can update an existing transaction by using its transaction ID.

4. Delete a transaction

   Users can delete a specific transaction by using its transaction ID.

5. Calculate total income

   The smart contract can calculate all transactions with the `income` type.

6. Calculate total expenses

   The smart contract can calculate all transactions with the `expense` type.

7. Check final balance

   The smart contract can calculate the final balance by subtracting total expenses from total income.

## Data Structure

Each transaction contains the following data:

- `id`: Unique ID for each transaction
- `title`: Transaction title or name
- `amount`: Transaction amount
- `category`: Transaction category
- `transaction_type`: Transaction type, either `income` or `expense`

## Example Categories

Some example categories that can be used:

- `food`
- `transport`
- `shopping`
- `salary`
- `entertainment`
- `education`

## Smart Contract Functions

### get_transactions

Used to view all stored transactions.

### create_transaction

Used to add a new transaction.

Parameters:

- `title`
- `amount`
- `category`
- `transaction_type`

### update_transaction

Used to update transaction data by transaction ID.

Parameters:

- `id`
- `new_title`
- `new_amount`
- `new_category`
- `new_transaction_type`

### delete_transaction

Used to delete a transaction by transaction ID.

Parameter:

- `id`

### get_total_income

Used to calculate the total income.

### get_total_expense

Used to calculate the total expenses.

### get_balance

Used to calculate the final balance.

## Smart Contract ID

```text
CBHMFX5VWFSTBX25EEMX4LGZQZXW7L6YRMIJ4BHK4GRTM54H335TSCG5
