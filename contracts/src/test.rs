#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Address, symbol_short};
    use crate::{VeriHire, VeriHireClient};
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_verihire_happy_path() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VeriHire);
        let client = VeriHireClient::new(&env, &contract_id);

        let issuer = Address::generate(&env);
        let student = Address::generate(&env);
        let hash = symbol_short!("PUP_GRAD");

        env.mock_all_auths();
        client.register_cert(&issuer, &student, &hash);

        assert_eq!(client.verify_cert(&hash), true);
    }

    #[test]
    #[should_panic(expected = "Hash already registered!")]
    fn test_duplicate_fails() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VeriHire);
        let client = VeriHireClient::new(&env, &contract_id);
        env.mock_all_auths();

        let hash = symbol_short!("DUPE_ID");
        client.register_cert(&Address::generate(&env), &Address::generate(&env), &hash);
        client.register_cert(&Address::generate(&env), &Address::generate(&env), &hash);
    }
}