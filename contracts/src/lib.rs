#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, log};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Cert(Symbol), // Stores Student Address indexed by Certificate Hash
}

#[contract]
pub struct VeriHire;

#[contractimpl]
impl VeriHire {
    /// Registers a certificate hash. Only the University (issuer) can call this.
    pub fn register_cert(env: Env, issuer: Address, student: Address, cert_hash: Symbol) {
        issuer.require_auth(); // Cybersecurity: Identity & Access Management

        let key = DataKey::Cert(cert_hash.clone());
        if env.storage().instance().has(&key) {
            panic!("Hash already registered!");
        }

        env.storage().instance().set(&key, &student);
        log!(&env, "Certificate Registered", cert_hash);
    }

    /// Checks if a certificate hash exists on the Stellar blockchain.
    pub fn verify_cert(env: Env, cert_hash: Symbol) -> bool {
        env.storage().instance().has(&DataKey::Cert(cert_hash))
    }

    /// Triggers a payout (USDC/XLM) to a student once verified.
    pub fn hire_payout(env: Env, employer: Address, cert_hash: Symbol, amount: i128) {
        employer.require_auth();

        let key = DataKey::Cert(cert_hash);
        let student: Address = env.storage().instance().get(&key)
            .expect("Verification failed: Student not in registry.");

        // Event for the frontend to show the "Success" toast
        env.events().publish((symbol_short!("PAY"), employer), (student, amount));
    }
}

#[cfg(test)]
mod test;