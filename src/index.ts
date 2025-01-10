import { AgentState, Conversation, Tweet } from './types';
import { loadConfig } from './config/config';
import { TwitterService } from './services/TwitterService';
import { LLMService } from './services/LLMService';
import { WalletService } from './services/WalletService';
import { logger } from './utils/logger';

class AutonomousAgent {
  private config = loadConfig();
  private twitter: TwitterService;
  private llm: LLMService;
  private wallet: WalletService;
  private state: AgentState = {
    conversations: new Map(),
    userInteractions: new Map(),
  };

  constructor() {
    // Initialize services
    this.twitter = new TwitterService(
      this.config.twitter.apiKey,
      this.config.twitter.apiSecret,
      this.config.twitter.accessToken,
      this.config.twitter.accessTokenSecret
    );

    this.llm = new LLMService(
      this.config.llm.apiKey,
      this.config.llm.model,
      this.config.llm.maxTokens
    );

    this.wallet = new WalletService(
      this.config.wallet.privateKey,
      this.config.wallet.rpcUrl
    );
  }

  private async processMention(tweet: Tweet): Promise<void> {
    try {
      logger.info(`Processing mention: ${tweet.id}`);

      // Get or create conversation
      let conversation = this.state.conversations.get(tweet.conversationId || tweet.id);
      if (!conversation) {
        conversation = {
          id: tweet.conversationId || tweet.id,
          tweets: [],
          participants: [],
          context: [],
          lastInteraction: new Date(),
        };
        this.state.conversations.set(conversation.id, conversation);
      }

      // Update conversation
      conversation.tweets.push(tweet);
      conversation.lastInteraction = new Date();

      // Get user interaction data
      const userInteraction = this.state.userInteractions.get(tweet.authorId) || {
        userId: tweet.authorId,
        lastInteraction: new Date(),
        interactionCount: 0,
        transactions: [],
      };
      userInteraction.interactionCount++;
      userInteraction.lastInteraction = new Date();
      this.state.userInteractions.set(tweet.authorId, userInteraction);

      // Generate context for LLM
      const context = conversation.tweets.map(t => t.text);

      // Generate reply
      const reply = await this.llm.generateReply(
        tweet.text,
        context,
        'You are a helpful AI assistant on Twitter.'
      );

      // Send reply
      await this.twitter.replyToTweet(reply, tweet.id);
      logger.info(`Replied to tweet: ${tweet.id}`);

    } catch (error) {
      logger.error('Error processing mention:', error);
      throw error;
    }
  }

  async start(): Promise<void> {
    logger.info('Starting Autonomous AI Agent');
    
    while (true) {
      try {
        // Check mentions
        const mentions = await this.twitter.getMentions(this.state.lastCheckedMentionId);
        
        for (const mention of mentions) {
          await this.processMention(mention);
          this.state.lastCheckedMentionId = mention.id;
        }

        // Wait before next check
        await new Promise(resolve => setTimeout(resolve, 60000)); // 1 minute delay
      } catch (error) {
        logger.error('Error in main loop:', error);
        // Wait before retrying
        await new Promise(resolve => setTimeout(resolve, 300000)); // 5 minutes delay on error
      }
    }
  }
}

// Start the agent
const agent = new AutonomousAgent();
agent.start().catch(error => {
  logger.error('Fatal error:', error);
  process.exit(1);
});
