import { AIService } from './AIService';

interface DeepSeekConfig {
  apiKey: string;
  model: string;
}

export class DeepSeekService implements AIService {
  private config: DeepSeekConfig;

  constructor(config: DeepSeekConfig) {
    this.config = config;
  }

  async askQuestion(_question: string, _context?: string): Promise<string> {
    throw new Error('Not implemented');
  }
} 
