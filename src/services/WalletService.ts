import { ethers } from 'ethers';
import { WalletTransaction } from '../types';
import { logger } from '../utils/logger';

export class WalletService {
  private wallet: ethers.Wallet;
  private provider: ethers.JsonRpcProvider;

  constructor(privateKey: string, rpcUrl: string) {
    this.provider = new ethers.JsonRpcProvider(rpcUrl);
    this.wallet = new ethers.Wallet(privateKey, this.provider);
  }

  async getBalance(): Promise<string> {
    try {
      const balance = await this.wallet.getBalance();
      return ethers.formatEther(balance);
    } catch (error) {
      logger.error('Error getting wallet balance:', error);
      throw error;
    }
  }

  async sendTransaction(
    to: string,
    amount: string,
    currency: string = 'ETH'
  ): Promise<WalletTransaction> {
    try {
      const tx = await this.wallet.sendTransaction({
        to,
        value: ethers.parseEther(amount),
      });

      const transaction: WalletTransaction = {
        id: tx.hash,
        from: this.wallet.address,
        to,
        amount,
        currency,
        timestamp: new Date(),
        status: 'pending',
        txHash: tx.hash,
      };

      // Wait for transaction confirmation
      const receipt = await tx.wait();
      
      return {
        ...transaction,
        status: receipt.status === 1 ? 'completed' : 'failed',
      };
    } catch (error) {
      logger.error('Error sending transaction:', error);
      throw error;
    }
  }

  async getTransactionStatus(txHash: string): Promise<'pending' | 'completed' | 'failed'> {
    try {
      const tx = await this.provider.getTransaction(txHash);
      if (!tx) {
        return 'failed';
      }

      const receipt = await tx.wait();
      return receipt.status === 1 ? 'completed' : 'failed';
    } catch (error) {
      logger.error('Error getting transaction status:', error);
      throw error;
    }
  }

  getAddress(): string {
    return this.wallet.address;
  }
}
