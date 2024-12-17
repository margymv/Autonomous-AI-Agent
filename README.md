# Autonomous AI Agent

Welcome to the **Autonomous AI Agent** project by [@btb_finance](https://twitter.com/btb_finance) and the [BTBFinance](https://t.me/BTBFinance) community.  
This project aims to create an autonomous AI-driven agent that can manage social media accounts (starting with Twitter), interact with users, remember context, handle tasks like sending tips (cryptocurrency microtransactions), and more—all with minimal human intervention. The long-term vision is to expand beyond Twitter to multiple social media platforms.

We're building this in **Rust** for performance, reliability, and security.

**Repository:** [https://github.com/btb-finance/Autonomous-AI-Agent](https://github.com/btb-finance/Autonomous-AI-Agent)

## Project Goals

1. **Autonomous Social Media Management**  
   The agent should:
   - Read mentions, replies, and direct messages
   - Generate and publish tweets autonomously using integrated LLMs (e.g., Claude API)
   - Maintain a rolling memory and conversational context
   - Monitor specific topics and hashtags (e.g., $BTB)
   - Engage with community through likes and replies
   
2. **Cryptocurrency Tipping and Transactions**  
   The agent can:
   - Generate and securely store a wallet address and private keys
   - Send small crypto tips to users who meet certain criteria
   - Interact with blockchain APIs or libraries to sign and broadcast transactions
   - Track transaction history and user interactions
   - Implement secure key management and encryption
   
3. **Scalable and Extensible Architecture**  
   Our code aims to:
   - Be easily extensible to other social media platforms
   - Allow integration with various LLM providers and conversation strategies
   - Encourage community contributions and improvements
   - Support multiple blockchain networks and token standards

## Key Features

- **Rust Backend:**  
  - Leveraging Rust for performance, memory safety, and reliable concurrency
  - Async runtime with Tokio for efficient I/O operations
  - Strong type system and ownership model for reliability
  
- **LLM Integration (e.g., Claude):**  
  - Use language model APIs for natural language understanding and generation
  - Context-aware responses with conversation history
  - Custom prompt engineering for specific use cases
  - Fallback mechanisms for API failures

- **Secure Storage of Keys and State:**  
  - Private keys and secrets are never hardcoded
  - AES-GCM encryption for sensitive data at rest
  - Secure key derivation and management
  - Regular state backups and recovery mechanisms

- **Event-Driven / Polling Architecture:**  
  - Periodically fetch mentions/DMs from Twitter
  - Process each event through the LLM for intelligent responses
  - Rate limiting and exponential backoff
  - Error handling and retry mechanisms

- **Compliance and Safety Controls:**  
  - Guardrails at the prompt and code level
  - Content filtering and moderation
  - Rate limiting and anti-spam measures
  - Audit logging for all operations

## Project Status

- **Early Stage:**  
  Currently implemented:
  - Basic Twitter API integration with OAuth2
  - LLM integration with Claude
  - Cryptocurrency wallet management
  - State persistence and conversation history
  - Rate limiting and error handling
  
- **Contributions Needed:**  
  Priority areas:
  - Enhanced conversation strategies
  - Additional social media platform integrations
  - Improved security measures
  - Testing and documentation
  - UI/UX for monitoring and configuration

## Getting Started

### Prerequisites

- **Rust and Cargo:**  
  - Rust (latest stable version) from [https://rustup.rs/](https://rustup.rs/)
  - Required crates listed in `Cargo.toml`

- **Twitter Developer Account:**  
  Required credentials:
  - API Key and Secret
  - OAuth 2.0 Client ID and Secret
  - App permissions for read/write access

- **Claude API Key (or another LLM):**  
  - Claude API key from [Anthropic](https://www.anthropic.com/)
  - Rate limits and usage quotas consideration
  - Backup LLM provider configuration (optional)

- **Environment Variables:**
  Create a `.env` file with:
  ```env
  TWITTER_API_KEY=your_twitter_api_key
  TWITTER_API_SECRET=your_twitter_api_secret
  TWITTER_ACCESS_TOKEN=your_access_token
  TWITTER_ACCESS_SECRET=your_access_secret
  CLAUDE_API_KEY=your_claude_api_key
  WALLET_PRIVATE_KEY=your_wallet_private_key
  ETH_RPC_URL=your_ethereum_node_url
  ENCRYPTION_KEY=your_encryption_key
  ```

### Installation

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/btb-finance/Autonomous-AI-Agent.git
   cd Autonomous-AI-Agent
   ```

2. **Build the Project:**
   ```bash
   # Development build
   cargo build

   # Production build
   cargo build --release
   ```

3. **Run the Agent:**
   ```bash
   # Development
   cargo run

   # Production
   cargo run --release
   ```

## Configuration

Configuration files are organized as follows:
- `.env` - Environment variables and secrets
- `config/` - Configuration files
  - `twitter.json` - Twitter API settings
  - `llm.json` - LLM provider settings
  - `wallet.json` - Blockchain configuration
  - `state.json` - Agent state persistence

## Project Structure

```
src/
├── main.rs           # Application entry point
├── api.rs            # Twitter API interactions
├── auth.rs           # Authentication handling
├── config.rs         # Configuration management
├── llm_client.rs     # LLM integration
├── models.rs         # Data structures
├── state.rs          # State management
├── wallet.rs         # Crypto operations
└── utils/
    ├── crypto.rs     # Encryption utilities
    ├── logging.rs    # Logging setup
    └── retry.rs      # Retry mechanisms
```

## Development Guidelines

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Run `cargo clippy` for linting
- Maintain comprehensive documentation
- Write unit tests for new features

### Security Best Practices
- Never commit sensitive data
- Use environment variables for secrets
- Implement proper error handling
- Log securely (no sensitive data)
- Regular security audits

### Testing
- Unit tests for core functionality
- Integration tests for API interactions
- Mock external services in tests
- Regular security scanning

## Roadmap

### Phase 1 - Core Infrastructure
- ✅ Basic Twitter API integration
- ✅ LLM integration
- ✅ Wallet management
- 🔄 State persistence
- 🔄 Rate limiting

### Phase 2 - Enhanced Features
- ⏳ Advanced conversation strategies
- ⏳ Multi-platform support
- ⏳ Improved security
- ⏳ Web interface
- ⏳ Analytics

### Phase 3 - Scale & Optimize
- 📋 Performance optimization
- 📋 Load balancing
- 📋 Monitoring system
- 📋 Backup systems
- 📋 Documentation

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

Quick start:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## Community

- Twitter: [@btb_finance](https://twitter.com/btb_finance)
- Telegram: [BTBFinance](https://t.me/BTBFinance)
- Discord: [Coming Soon]

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Twitter API Documentation
- Claude API Documentation
- Ethereum Documentation
- Rust Community
- BTB Finance Community

---

Built with ❤️ by the BTB Finance community.
