# Autonomous AI Agent

Welcome to the **Autonomous AI Agent** project by [@btb_finance](https://twitter.com/btb_finance) and the [BTBFinance](https://t.me/BTBFinance) community.  
This project aims to create an autonomous AI-driven agent that can manage social media accounts (starting with Twitter), interact with users, remember context, handle tasks like sending tips (cryptocurrency microtransactions), and more—all with minimal human intervention. The long-term vision is to expand beyond Twitter to multiple social media platforms.

We're building this in **TypeScript** for type safety, extensive ecosystem support, and rapid development capabilities.

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
   - Interact with blockchain APIs using ethers.js
   - Track transaction history and user interactions
   - Implement secure key management and encryption
   
3. **Scalable and Extensible Architecture**  
   Our code aims to:
   - Be easily extensible to other social media platforms
   - Allow integration with various LLM providers and conversation strategies
   - Encourage community contributions and improvements
   - Support multiple blockchain networks and token standards

## Key Features

- **TypeScript Backend:**  
  - Strong type system for enhanced code reliability
  - Modern async/await patterns for efficient I/O operations
  - Rich ecosystem of NPM packages
  - Easy integration with Node.js
  
- **LLM Integration (Claude):**  
  - Use language model APIs for natural language understanding and generation
  - Context-aware responses with conversation history
  - Custom prompt engineering for specific use cases
  - Fallback mechanisms for API failures

- **Secure Storage of Keys and State:**  
  - Environment-based configuration using dotenv
  - Private keys and secrets are never hardcoded
  - Secure key management practices
  - Regular state persistence

- **Event-Driven Architecture:**  
  - Periodically fetch mentions from Twitter
  - Process each event through the LLM for intelligent responses
  - Rate limiting and exponential backoff
  - Error handling and retry mechanisms

- **Compliance and Safety Controls:**  
  - Guardrails at the prompt and code level
  - Content filtering and moderation
  - Rate limiting and anti-spam measures
  - Audit logging for all operations

## Project Status

- **Current Implementation:**  
  Features implemented:
  - Twitter API integration using twitter-api-v2
  - Claude API integration for LLM capabilities
  - Ethereum wallet management with ethers.js
  - State management and conversation tracking
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

- **Node.js and npm:**  
  - Node.js (v16 or later) from [https://nodejs.org/](https://nodejs.org/)
  - Required packages listed in `package.json`

- **Twitter Developer Account:**  
  Required credentials:
  - API Key and Secret
  - Access Token and Secret
  - App permissions for read/write access

- **Claude API Key:**  
  - Claude API key from [Anthropic](https://www.anthropic.com/)
  - Rate limits and usage quotas consideration

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/btb-finance/Autonomous-AI-Agent.git
   cd Autonomous-AI-Agent
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Set up environment variables:
   - Copy `.env.example` to `.env`
   - Fill in your API keys and credentials

4. Build the project:
   ```bash
   npm run build
   ```

5. Start the agent:
   ```bash
   npm start
   ```

For development:
```bash
npm run dev
```

## Project Structure

```
src/
├── index.ts              # Main application entry point
├── services/            # Core services
│   ├── TwitterApi.ts    # Twitter API integration
│   ├── LLMClient.ts     # Claude API integration
│   └── WalletManager.ts # Ethereum wallet management
└── types/              # TypeScript type definitions
    └── index.ts        # Shared types and interfaces
```

## Configuration

The application uses environment variables for configuration:

- `TWITTER_API_KEY` - Twitter API key
- `TWITTER_API_SECRET` - Twitter API secret
- `TWITTER_ACCESS_TOKEN` - Twitter access token
- `TWITTER_ACCESS_TOKEN_SECRET` - Twitter access token secret
- `CLAUDE_API_KEY` - Claude API key
- `WALLET_PRIVATE_KEY` - Ethereum wallet private key
- `ETH_RPC_URL` - Ethereum RPC URL

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
