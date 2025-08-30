import path from 'path';
import { config } from 'dotenv';
import { TwitterService } from './services/TwitterService';
import { OpenRouterService } from './services/OpenRouterService';
import { KnowledgeBaseService } from './services/KnowledgeBaseService';
import { BTBTweetService } from './services/BTBTweetService';
import { logger } from './utils/logger';

// Load environment variables
config();

async function main() {
  try {
    logger.info('Loading configuration...');

    // Initialize services
    const twitterService = new TwitterService(
      process.env.TWITTER_API_KEY!,
      process.env.TWITTER_API_SECRET!,
      process.env.TWITTER_ACCESS_TOKEN!,
      process.env.TWITTER_ACCESS_TOKEN_SECRET!,
      process.env.TWITTER_BEARER_TOKEN!
    );

    const openRouterService = new OpenRouterService(
      process.env.OPENROUTER_API_KEY!,
      process.env.OPENROUTER_MODEL || 'anthropic/claude-3.5-sonnet'
    );

    const knowledgeBaseService = new KnowledgeBaseService(
      path.join(__dirname, '..', 'knowledge_base', 'btb_info.txt')
    );

    // Initialize BTB tweet service
    const btbTweetService = new BTBTweetService(
      twitterService,
      openRouterService,
      knowledgeBaseService
    );

    logger.info('Starting Autonomous AI Agent');

    // Start processing tweets every 5 minutes
    await btbTweetService.startProcessing(300000); // 5 minutes = 300000ms
  } catch (error) {
    logger.error('Error in main loop:', error);
    process.exit(1);
  }
}

main();
