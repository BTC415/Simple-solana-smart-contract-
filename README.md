# Solana NFT Minting Smart Contract
<img src="https://img.shields.io/badge/Solana-Practice-green">

A robust implementation of NFT minting functionality on Solana blockchain using Anchor framework and Rust.

## Project Overview

This project demonstrates a secure and efficient way to mint NFTs on Solana blockchain with the following components:

- Smart Contract written in Rust using Anchor framework
- Frontend interface built with React
- Complete NFT minting functionality with metadata handling

## Technical Stack

- **Backend**:
  - Rust
  - Anchor Framework
  - Solana Program Library (SPL)
  - Token Program Integration

- **Frontend**:
  - React.js
  - Web3.js
  - Solana Wallet Adapter

## Smart Contract Features

- Secure mint authority management
- Token metadata program integration
- System program interactions
- Error handling for mint and metadata operations
- Token account management


## Getting Started

1. Install Dependencies:
```bash
npm install
```

2. Build the smart contract:
```bash
anchor build
```

3. Deploy the smart contract to Solana Network:
```bash
anchor deploy
```

4. Run the frontend:
```bash
cd frontend && npm start
```

## Security Considerations

- Implemented proper account validation
- Secure mint authority checks
- Protected metadata account creation
- Error handling for failed operations

## Contribution

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a pull request

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
 