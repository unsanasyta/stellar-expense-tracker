# ExpenseTracker

## Deskripsi Projek

ExpenseTracker adalah smart contract berbasis Stellar Soroban yang digunakan untuk mencatat pemasukan dan pengeluaran secara sederhana.

Project ini dibuat agar pengguna dapat menyimpan data transaksi seperti judul transaksi, jumlah uang, kategori, dan jenis transaksi. Jenis transaksi dapat berupa `income` untuk pemasukan atau `expense` untuk pengeluaran.

Dengan ExpenseTracker, pengguna dapat melihat daftar transaksi, menghitung total pemasukan, menghitung total pengeluaran, serta mengetahui saldo akhir berdasarkan data transaksi yang sudah dibuat.

## Visi Projek

Visi dari ExpenseTracker adalah membantu pengguna mencatat dan memantau keuangan pribadi secara sederhana, transparan, dan terdesentralisasi menggunakan teknologi blockchain.

Project ini diharapkan dapat menjadi dasar untuk pengembangan aplikasi keuangan yang lebih lengkap di masa depan, seperti budgeting, laporan keuangan, pengelompokan transaksi, dan pencatatan histori keuangan berbasis smart contract.

## List Fitur Projek

ExpenseTracker memiliki beberapa fitur utama, yaitu:

1. Menambahkan transaksi baru

   Pengguna dapat membuat transaksi baru dengan memasukkan judul, jumlah uang, kategori, dan jenis transaksi.

2. Melihat semua transaksi

   Pengguna dapat melihat seluruh daftar transaksi yang sudah tersimpan di smart contract.

3. Mengubah transaksi

   Pengguna dapat memperbarui data transaksi berdasarkan ID transaksi yang sudah dibuat.

4. Menghapus transaksi

   Pengguna dapat menghapus transaksi tertentu berdasarkan ID transaksi.

5. Menghitung total pemasukan

   Smart contract dapat menghitung seluruh transaksi dengan jenis `income`.

6. Menghitung total pengeluaran

   Smart contract dapat menghitung seluruh transaksi dengan jenis `expense`.

7. Melihat saldo akhir

   Smart contract dapat menghitung saldo akhir dari total pemasukan dikurangi total pengeluaran.

## Struktur Data

Setiap transaksi memiliki struktur data sebagai berikut:

- `id`: ID unik untuk setiap transaksi
- `title`: Judul atau nama transaksi
- `amount`: Jumlah uang pada transaksi
- `category`: Kategori transaksi
- `transaction_type`: Jenis transaksi, yaitu `income` atau `expense`

## Contoh Kategori

Beberapa contoh kategori yang dapat digunakan:

- `food`
- `transport`
- `shopping`
- `salary`
- `entertainment`
- `education`

## Fungsi Smart Contract

### get_transactions

Digunakan untuk melihat semua transaksi yang tersimpan.

### create_transaction

Digunakan untuk menambahkan transaksi baru.

Parameter:

- `title`
- `amount`
- `category`
- `transaction_type`

### update_transaction

Digunakan untuk mengubah data transaksi berdasarkan ID.

Parameter:

- `id`
- `new_title`
- `new_amount`
- `new_category`
- `new_transaction_type`

### delete_transaction

Digunakan untuk menghapus transaksi berdasarkan ID.

Parameter:

- `id`

### get_total_income

Digunakan untuk menghitung total pemasukan.

### get_total_expense

Digunakan untuk menghitung total pengeluaran.

### get_balance

Digunakan untuk menghitung saldo akhir.

## ID Smart Contract

```text
CBHMFX5VWFSTBX25EEMX4LGZQZXW7L6YRMIJ4BHK4GRTM54H335TSCG5