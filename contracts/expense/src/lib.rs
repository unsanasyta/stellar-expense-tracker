#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

// Struktur data untuk menyimpan transaksi
#[contracttype]
#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: u64,
    pub title: String,
    pub amount: i128,
    pub category: String,
    pub transaction_type: String, // "income" atau "expense"
}

// Storage key untuk data transaksi
const TRANSACTION_DATA: Symbol = symbol_short!("TX_DATA");

#[contract]
pub struct ExpenseTracker;

#[contractimpl]
impl ExpenseTracker {
    // Fungsi untuk melihat semua transaksi
    pub fn get_transactions(env: Env) -> Vec<Transaction> {
        env.storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk menambahkan transaksi baru
    pub fn create_transaction(
        env: Env,
        title: String,
        amount: i128,
        category: String,
        transaction_type: String,
    ) -> String {
        // 1. Ambil data transaksi dari storage
        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Buat object transaksi baru
        let transaction = Transaction {
            id: env.prng().gen::<u64>(),
            title,
            amount,
            category,
            transaction_type,
        };

        // 3. Tambahkan transaksi baru ke list lama
        transactions.push_back(transaction);

        // 4. Simpan kembali ke storage
        env.storage()
            .instance()
            .set(&TRANSACTION_DATA, &transactions);

        String::from_str(&env, "Transaksi berhasil ditambahkan")
    }

    // Fungsi untuk update transaksi berdasarkan id
    pub fn update_transaction(
        env: Env,
        id: u64,
        new_title: String,
        new_amount: i128,
        new_category: String,
        new_transaction_type: String,
    ) -> String {
        // 1. Ambil data transaksi dari storage
        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari transaksi berdasarkan id
        for i in 0..transactions.len() {
            let transaction = transactions.get(i).unwrap();

            if transaction.id == id {
                let updated_transaction = Transaction {
                    id,
                    title: new_title,
                    amount: new_amount,
                    category: new_category,
                    transaction_type: new_transaction_type,
                };

                transactions.set(i, updated_transaction);

                env.storage()
                    .instance()
                    .set(&TRANSACTION_DATA, &transactions);

                return String::from_str(&env, "Transaksi berhasil diupdate");
            }
        }

        String::from_str(&env, "Transaksi tidak ditemukan")
    }

    // Fungsi untuk menghapus transaksi berdasarkan id
    pub fn delete_transaction(env: Env, id: u64) -> String {
        // 1. Ambil data transaksi dari storage
        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari index transaksi yang akan dihapus
        for i in 0..transactions.len() {
            if transactions.get(i).unwrap().id == id {
                transactions.remove(i);

                env.storage()
                    .instance()
                    .set(&TRANSACTION_DATA, &transactions);

                return String::from_str(&env, "Transaksi berhasil dihapus");
            }
        }

        String::from_str(&env, "Transaksi tidak ditemukan")
    }

    // Fungsi tambahan untuk menghitung total pemasukan
    pub fn get_total_income(env: Env) -> i128 {
        let transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        let mut total: i128 = 0;
        let income_type = String::from_str(&env, "income");

        for i in 0..transactions.len() {
            let transaction = transactions.get(i).unwrap();

            if transaction.transaction_type == income_type {
                total += transaction.amount;
            }
        }

        total
    }

    // Fungsi tambahan untuk menghitung total pengeluaran
    pub fn get_total_expense(env: Env) -> i128 {
        let transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        let mut total: i128 = 0;
        let expense_type = String::from_str(&env, "expense");

        for i in 0..transactions.len() {
            let transaction = transactions.get(i).unwrap();

            if transaction.transaction_type == expense_type {
                total += transaction.amount;
            }
        }

        total
    }

    // Fungsi tambahan untuk melihat saldo akhir
    pub fn get_balance(env: Env) -> i128 {
        let income = Self::get_total_income(env.clone());
        let expense = Self::get_total_expense(env);

        income - expense
    }
}

mod test;