#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan kontak
#[contracttype]
#[derive(Clone, Debug)]
pub struct Contact {
    pub id: u64,
    pub name: String,
    pub phone_number: String,
}

// Storage key untuk data kontak
const CONTACT_DATA: Symbol = symbol_short!("CONTACT");

#[contract]
pub struct ContactsContract;

#[contractimpl]
impl ContactsContract {
    // Fungsi untuk mendapatkan semua kontak
    pub fn get_contacts(env: Env) -> Vec<Contact> {
        // 1. ambil data kontak dari storage
        return env.storage().instance().get(&CONTACT_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat kontak baru
    pub fn create_contact(env: Env, name: String, phone_number: String) -> String {
        // 1. ambil data kontak dari storage
        let mut contacts: Vec<Contact> = env.storage().instance().get(&CONTACT_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object kontak baru
        let contact = Contact {
            id: env.prng().gen::<u64>(),
            name: name,
            phone_number: phone_number,
        };
        
        // 3. tambahkan kontak baru ke kontak lama
        contacts.push_back(contact);
        
        // 4. simpan kontak ke storage
        env.storage().instance().set(&CONTACT_DATA, &contacts);
        
        return String::from_str(&env, "Kontak berhasil ditambahkan");
    }

    // Fungsi untuk menghapus kontak berdasarkan id
    pub fn delete_contact(env: Env, id: u64) -> String {
        // 1. ambil data kontak dari storage 
        let mut contacts: Vec<Contact> = env.storage().instance().get(&CONTACT_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index kontak yang akan dihapus menggunakan perulangan
        for i in 0..contacts.len() {
            if contacts.get(i).unwrap().id == id {
                contacts.remove(i);

                env.storage().instance().set(&CONTACT_DATA, &contacts);
                return String::from_str(&env, "Berhasil hapus kontak");
            }
        }

        return String::from_str(&env, "Kontak tidak ditemukan")
    }
}

mod test;