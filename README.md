# VeriHire - Soroban Certificate Verification Contract

A Soroban smart contract deployed on the Stellar network for secure academic certificate verification and hiring payouts.

## 🚀 Live Contract

**Testnet Contract Address**: `CBWLBDLCFA35HQAU4LT43MRI5W7FYMT7P4UWPXRRGST7RB5IIDFQWYQA`

**Stellar Lab Explorer**: [View Contract](https://lab.stellar.org/r/testnet/contract/CBWLBDLCFA35HQAU4LT43MRI5W7FYMT7P4UWPXRRGST7RB5IIDFQWYQA)

![Alt text](Gallery)


## 📋 Overview

VeriHire enables universities to register student certificates on the Stellar blockchain and employers to verify them before making hiring payments. This ensures immutable, decentralized certificate verification.

### Key Features
- ✅ **Certificate Registration**: Universities can register student certificates with unique hashes
- ✅ **Certificate Verification**: Employers can verify certificate authenticity
- ✅ **Secure Payouts**: Automated USDC/XLM payments upon successful verification
- ✅ **Immutable Records**: All data stored on Stellar's decentralized ledger

## 🏗️ Contract Functions

### `register_cert(issuer: Address, student: Address, cert_hash: Symbol)`
Registers a certificate hash. Only the issuer (university) can call this.
- **Parameters**:
  - `issuer`: University account address
  - `student`: Student account address
  - `cert_hash`: Unique certificate identifier
- **Security**: Requires issuer authentication

### `verify_cert(cert_hash: Symbol) -> bool`
Checks if a certificate hash exists in the registry.
- **Parameters**:
  - `cert_hash`: Certificate hash to verify
- **Returns**: `true` if certificate exists, `false` otherwise

### `hire_payout(employer: Address, cert_hash: Symbol, amount: i128)`
Triggers a payout to the student after verification.
- **Parameters**:
  - `employer`: Employer account address
  - `cert_hash`: Certificate hash for verification
  - `amount`: Payment amount in stroops
- **Security**: Requires employer authentication and valid certificate

## 🛠️ Development Setup

### Prerequisites
- Rust (latest stable)
- Stellar CLI
- Git

### Installation
```bash
# Clone the repository
git clone <your-repo-url>
cd VeriHire-PHP

# Install WASM target
rustup target add wasm32-unknown-unknown

# Navigate to contract directory
cd contracts
```

### Building
```bash
# Build for deployment
stellar contract build

# Or use cargo for testing
cargo build
```

### Testing
```bash
# Run unit tests
cargo test

# All tests should pass: 2/2 ✅
```

### Deployment
```bash
# Generate keys (testnet)
stellar keys generate --global my-key --network testnet
stellar keys fund my-key --network testnet

# Build and deploy
stellar contract build
stellar contract deploy \
  --wasm target/wasm32v1-none/release/verihire_php.wasm \
  --source my-key \
  --network testnet
```

## 🧪 Testing the Contract

### CLI Testing
```bash
# Verify non-existent certificate (should return false)
stellar contract invoke \
  --id CBWLBDLCFA35HQAU4LT43MRI5W7FYMT7P4UWPXRRGST7RB5IIDFQWYQA \
  --source my-key \
  --network testnet \
  -- verify_cert --cert_hash TEST_HASH

# Register a certificate
stellar contract invoke \
  --id CBWLBDLCFA35HQAU4LT43MRI5W7FYMT7P4UWPXRRGST7RB5IIDFQWYQA \
  --source my-key \
  --network testnet \
  -- register_cert \
  --issuer <university-address> \
  --student <student-address> \
  --cert_hash PUP_GRAD \
  --send=yes

# Verify registered certificate (should return true)
stellar contract invoke \
  --id CBWLBDLCFA35HQAU4LT43MRI5W7FYMT7P4UWPXRRGST7RB5IIDFQWYQA \
  --source my-key \
  --network testnet \
  -- verify_cert --cert_hash PUP_GRAD
```

### Web Interface Testing
Use the [Stellar Lab Contract Explorer](https://lab.stellar.org/r/testnet/contract/CBWLBDLCFA35HQAU4LT43MRI5W7FYMT7P4UWPXRRGST7RB5IIDFQWYQA) to invoke functions directly from your browser.

## 📁 Project Structure
```
VeriHire-PHP/
├── contracts/                 # Soroban contract
│   ├── src/
│   │   ├── lib.rs            # Main contract logic
│   │   └── test.rs           # Unit tests
│   ├── Cargo.toml            # Rust dependencies
│   └── target/               # Build artifacts (ignored)
├── .gitignore               # Git ignore rules
└── README.md                # This file
```

## 🔒 Security Features
- **Identity & Access Management**: Functions require proper authentication
- **Immutable Storage**: Certificate data cannot be altered once registered
- **Duplicate Prevention**: Contract prevents registering the same hash twice
- **Stellar Network Security**: Benefits from Stellar's consensus and cryptography

## 🤝 Contributing
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## 📜 License
This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments
- Built with [Soroban SDK](https://soroban.stellar.org/)
- Deployed on [Stellar Network](https://stellar.org/)
- Inspired by the need for decentralized academic credential verification

---

**Status**: ✅ Deployed on Stellar Testnet | ✅ Tests Passing | ✅ Source Code Committed</content>
<parameter name="filePath">c:\Users\lance\verihire\VeriHire-PHP\README.md
