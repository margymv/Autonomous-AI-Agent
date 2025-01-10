import { config as dotenvConfig } from 'dotenv';
import { AgentConfig } from '../types';

dotenvConfig();

function validateConfig(config: Partial<AgentConfig>): config is AgentConfig {
  const requiredFields = [
    'twitter.apiKey',
    'twitter.apiSecret',
    'twitter.accessToken',
    'twitter.accessTokenSecret',
    'llm.apiKey',
    'wallet.privateKey',
    'wallet.rpcUrl'
  ];

  for (const field of requiredFields) {
    const value = field.split('.').reduce((obj, key) => obj?.[key], config as any);
    if (!value) {
      throw new Error(`Missing required configuration: ${field}`);
    }
  }

  return true;
}

export function loadConfig(): AgentConfig {
  const config: AgentConfig = {
    twitter: {
      apiKey: process.env.TWITTER_API_KEY!,
      apiSecret: process.env.TWITTER_API_SECRET!,
      accessToken: process.env.TWITTER_ACCESS_TOKEN!,
      accessTokenSecret: process.env.TWITTER_ACCESS_TOKEN_SECRET!,
    },
    llm: {
      provider: 'claude',
      apiKey: process.env.CLAUDE_API_KEY!,
      model: 'claude-2',
      maxTokens: 1000,
    },
    wallet: {
      network: process.env.ETH_NETWORK || 'mainnet',
      rpcUrl: process.env.ETH_RPC_URL!,
      privateKey: process.env.WALLET_PRIVATE_KEY!,
    },
    monitoring: {
      logLevel: (process.env.LOG_LEVEL || 'info') as AgentConfig['monitoring']['logLevel'],
      enableMetrics: process.env.ENABLE_METRICS === 'true',
    },
  };

  if (!validateConfig(config)) {
    throw new Error('Invalid configuration');
  }

  return config;
}
