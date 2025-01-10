import { TwitterApi as TwitterClient } from 'twitter-api-v2';
import { TwitterMention } from '../types';

export class TwitterApi {
  private client: TwitterClient;

  constructor(
    apiKey: string,
    apiSecret: string,
    accessToken: string,
    accessTokenSecret: string
  ) {
    this.client = new TwitterClient({
      appKey: apiKey,
      appSecret: apiSecret,
      accessToken: accessToken,
      accessSecret: accessTokenSecret,
    });
  }

  async getMentions(sinceId?: string): Promise<TwitterMention[]> {
    const mentions = await this.client.v2.mentions({
      since_id: sinceId,
      expansions: ['author_id'],
      'tweet.fields': ['author_id', 'text'],
    });

    return mentions.data.map(mention => ({
      id: mention.id,
      authorId: mention.author_id || '',
      text: mention.text,
    }));
  }

  async replyToTweet(text: string, replyToId: string): Promise<void> {
    await this.client.v2.reply(text, replyToId);
  }
}
