# Autonomous AI Twitter Agent

A Rust-based autonomous AI agent that monitors Twitter for specific topics, engages with users, and manages cryptocurrency tips.

## Features

- 🤖 Autonomous Twitter monitoring and interaction
- 🧠 AI-powered responses using Claude
- 💰 Cryptocurrency tipping functionality
- 🔒 Secure wallet management
- 🗄️ Persistent state management
- ⚡ Efficient async processing

## Prerequisites

- Rust (latest stable version)
- Twitter API credentials
- Claude API key
- Ethereum node access (for tipping functionality)

## Environment Variables

Create a `.env` file in the root directory with the following variables:

```env
TWITTER_API_KEY=your_twitter_api_key
TWITTER_API_SECRET=your_twitter_api_secret
CLAUDE_API_KEY=your_claude_api_key
WALLET_PRIVATE_KEY=your_wallet_private_key
ETH_RPC_URL=your_ethereum_node_url
```

## Project Structure

```
src/
├── main.rs         # Application entry point
├── api.rs          # Twitter API interactions
├── auth.rs         # Authentication handling
├── config.rs       # Configuration constants
├── llm_client.rs   # Claude API integration
├── models.rs       # Data structures
├── state.rs        # State management
└── wallet.rs       # Cryptocurrency operations
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/btb-finance/Autonomous-AI-Agent.git
cd Autonomous-AI-Agent
```

2. Install dependencies:
```bash
cargo build
```

3. Run the agent:
```bash
cargo run
```

## Configuration

### Twitter API Setup
1. Create a Twitter Developer account
2. Create a new app and get API credentials
3. Enable OAuth 2.0 and required permissions

### Claude API Setup
1. Obtain an API key from Anthropic
2. Add it to your environment variables

### Wallet Setup
1. Generate or import an Ethereum wallet
2. Fund the wallet for tipping functionality
3. Set up access to an Ethereum node (e.g., Infura)

## Usage

The agent automatically:
- Monitors Twitter for specific topics
- Generates contextual replies using Claude
- Manages user interactions and conversation history
- Handles cryptocurrency tips when requested

## Security Considerations

- Never commit `.env` file or private keys
- Use secure key management in production
- Implement rate limiting for API calls
- Monitor wallet balance and transactions

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Twitter API Documentation
- Claude API Documentation
- Ethereum Documentation
- Rust Community

## Contact

For questions or support, please open an issue on GitHub.
