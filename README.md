
# **NeuraGraph ($NGRPH)**

## **Description**

**NeuraGraph** is a decentralized AI knowledge graph built with Rust. It uses **IPFS** for distributed storage, **blockchain** for verification, and **cryptographic hashing** for data integrity. A command-line interface (CLI) tool allows you to easily add, validate, and search knowledge entries.

---

## **Features**

- 🗃 **Decentralized Storage**: Store knowledge entries securely on IPFS.
- 🔗 **Blockchain Integration**: Verify data integrity and transparency with blockchain.
- 🔒 **Cryptographic Verification**: Ensure data has not been tampered with using SHA-256 hashing.
- 🛠 **CLI Tool**: Simple commands to add, validate, search, and send data.
- 🔍 **Search Functionality**: Quickly find stored knowledge entries.

---

## **Installation**

Ensure you have **Rust** installed on your system. Then, clone the repository and build the project:

```bash
git clone https://github.com/neuragraph/neuragraph.git
cd neuragraph
cargo build --release
```

---

## **Usage**

### **1. Add a Knowledge Entry**

Create a JSON file (e.g., `entry.json`) with the following format:

```json
{
    "contributor": "WalletAddressHere",
    "data": "Sample knowledge entry",
    "signature": "SignedMessageHere"
}
```

Add the entry to NeuraGraph:

```bash
cargo run -- add entry.json
```

### **2. Validate an Entry**

Check if a knowledge entry is valid:

```bash
cargo run -- validate entry.json
```

### **3. Search for Entries**

Search entries by keywords:

```bash
cargo run -- search "keyword"
```

### **4. Send Data to Blockchain**

Simulate sending data to the blockchain:

```bash
cargo run -- send "data to send"
```

---

## **Project Structure**

```
NeuraGraph/
│-- src/
│   │-- main.rs          # Entry point of the application
│   │-- cli.rs           # CLI commands
│   │-- ipfs.rs          # IPFS integration
│   │-- consensus.rs     # Entry validation logic
│   │-- blockchain.rs    # Blockchain interaction (mock)
│   │-- crypto_utils.rs  # Cryptographic hashing
│   │-- utils.rs         # Helper functions
│-- Cargo.toml           # Project dependencies
│-- README.md            # Project documentation
└-- LICENSE              # License information
```

---

## **Contributing**

Contributions are welcome! Feel free to submit a pull request or open an issue to improve the project. We are open to all feedback and support. 

---

## **License**

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for more details.
