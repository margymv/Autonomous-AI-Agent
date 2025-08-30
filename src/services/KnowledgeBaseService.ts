import fs from 'fs/promises';
import path from 'path';
import { logger } from '../utils/logger';

export class KnowledgeBaseService {
  private knowledgeBasePath: string;

  constructor(knowledgeBasePath: string) {
    this.knowledgeBasePath = knowledgeBasePath;
  }

  async loadKnowledgeBase(): Promise<string> {
    try {
      logger.info('Loading knowledge base...');
      const content = await fs.readFile(this.knowledgeBasePath, 'utf-8');
      logger.info('Knowledge base loaded successfully', { contentLength: content.length });
      return content;
    } catch (error) {
      logger.error('Failed to load knowledge base:', error);
      throw error;
    }
  }

  async searchKnowledge(query: string): Promise<string> {
    try {
      const knowledgeBase = await this.loadKnowledgeBase();
      
      // Just return the knowledge base content
      // The prompt engineering happens in BTBTweetService
      return knowledgeBase;
    } catch (error) {
      logger.error('Failed to search knowledge base:', error);
      // Return empty string if knowledge base fails to load
      return '';
    }
  }
}
