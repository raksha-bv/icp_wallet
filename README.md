# ICRC Wallet

The **ICRC Wallet** is a project implemented in Rust, designed to interact with the Internet Computer's ICRC standard for token management. This repository contains the `Cargo.toml` configuration, the source code, and instructions to clone, build, and deploy the project.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)

## Features

- Implements the ICRC token standard
- Uses the Internet Computer (IC) SDK for deployment
- Provides serialization and deserialization with Serde
- Includes necessary dependencies for seamless integration with IC services

## Prerequisites

Ensure you have the following installed before starting:

- **Rust** and **Cargo**: Install via the [Rust official website](https://www.rust-lang.org/tools/install)
- **DFX** (The Internet Computer SDK): Install via the IC Developer Documentation
- **Git**: Install via [Git Downloads](https://git-scm.com/downloads)

## Getting Started

Follow the steps below to clone, build, and deploy the project.

### Cloning the Repository

To clone the repository, run the following commands in your terminal:

```bash
git clone https://github.com/your-username/icrc_wallet.git
cd icrc_wallet
```

### Building the Project

Build the project using Cargo:

```bash
cargo build
```

### Deploying the Canister

Deploy the canister to the Internet Computer using DFX:

1. Start the local DFX replica:
   ```bash
   dfx start --background
   ```

2. Deploy the canister:
   ```bash
   dfx deploy
   ```

### Interacting with the Canister

Once deployed, you can interact with the canister using DFX commands or a frontend interface.

## Configuration

The project uses the following `Cargo.toml` configuration:

```toml
[package]
name = "icrc_wallet"
version = "0.1.0"
edition = "2021"

[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
ic-cdk-macros = "0.8.1"
icrc-ledger-types = "0.1.1"
serde = { version = "1.0", features = ["derive"] }

[lib]
crate-type = ["cdylib"]
```

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a new branch:
   ```bash
   git checkout -b feature-name
   ```
3. Make your changes and commit them:
   ```bash
   git commit -m "Add new feature: feature-name"
   ```
4. Push to your branch:
   ```bash
   git push origin feature-name
   ```
5. Create a pull request

## License

This project is licensed under the MIT License. Feel free to use it as you wish!
