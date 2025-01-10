import dotenv from 'dotenv';
import { TwitterApi } from './services/TwitterApi';
import { LLMClient } from './services/LLMClient';
import { WalletManager } from './services/WalletManager';
import { BotState, Conversation } from './types';

// Load environment variables
dotenv.config();

class Bot {
  private twitterApi: TwitterApi;
  private llmClient: LLMClient;
  private walletManager: WalletManager;
  private state: BotState = {
    lastCheckedMentionId: null,
    conversations: new Map<string, Conversation>(),
    userInteractions: new Map(),
  };

  constructor() {
    // Initialize services
    this.twitterApi = new TwitterApi(
      process.env.TWITTER_API_KEY!,
      process.env.TWITTER_API_SECRET!,
      process.env.TWITTER_ACCESS_TOKEN!,
      process.env.TWITTER_ACCESS_TOKEN_SECRET!
    );

    this.llmClient = new LLMClient(
      process.env.CLAUDE_API_KEY!
    );

    this.walletManager = new WalletManager(
      process.env.WALLET_PRIVATE_KEY!,
      process.env.ETH_RPC_URL!
    );
  }

  async processMentions(): Promise<void> {
    try {
      const mentions = await this.twitterApi.getMentions(this.state.lastCheckedMentionId || undefined);

      for (const mention of mentions) {
        console.log(`Processing mention: ${mention.id}`);

        // Generate reply using LLM
        const reply = await this.llmClient.generateReply(
          mention.text,
          [`User: ${mention.authorId}`]
        );

        // Reply to tweet
        await this.twitterApi.replyToTweet(reply, mention.id);

        // Update state
        this.state.conversations.set(mention.id, {
          mentionId: mention.id,
          authorId: mention.authorId,
          text: mention.text,
          timestamp: new Date(),
        });

        // Update user interaction
        const userInteraction = this.state.userInteractions.get(mention.authorId) || {
          userId: mention.authorId,
          interactionCount: 0,
          lastInteraction: new Date(),
        };
        userInteraction.interactionCount++;
        userInteraction.lastInteraction = new Date();
        this.state.userInteractions.set(mention.authorId, userInteraction);

        // Update last checked mention ID
        this.state.lastCheckedMentionId = mention.id;
      }
    } catch (error) {
      console.error('Error processing mentions:', error);
    }
  }

  async start(): Promise<void> {
    console.log('Starting Autonomous AI Twitter Agent');
    
    while (true) {
      await this.processMentions();
      await new Promise(resolve => setTimeout(resolve, 60000)); // Wait for 60 seconds
    }
  }
}

// Start the bot
const bot = new Bot();
bot.start().catch(console.error);
