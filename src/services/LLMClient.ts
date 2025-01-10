import axios from 'axios';

export class LLMClient {
  private apiKey: string;
  private baseUrl = 'https://api.anthropic.com/v1/messages';

  constructor(apiKey: string) {
    this.apiKey = apiKey;
  }

  async generateReply(text: string, context: string[]): Promise<string> {
    try {
      const response = await axios.post(
        this.baseUrl,
        {
          model: 'claude-2',
          messages: [
            {
              role: 'user',
              content: `Context: ${context.join('\n')}\n\nMessage: ${text}`,
            },
          ],
          max_tokens: 1000,
        },
        {
          headers: {
            'Content-Type': 'application/json',
            'x-api-key': this.apiKey,
          },
        }
      );

      return response.data.content[0].text;
    } catch (error) {
      console.error('Error generating reply:', error);
      throw error;
    }
  }
}
