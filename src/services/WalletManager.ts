import { ethers } from 'ethers';

export class WalletManager {
  private wallet: ethers.Wallet;
  private provider: ethers.JsonRpcProvider;

  constructor(privateKey: string, rpcUrl: string) {
    this.provider = new ethers.JsonRpcProvider(rpcUrl);
    this.wallet = new ethers.Wallet(privateKey, this.provider);
  }

  async getBalance(): Promise<string> {
    const balance = await this.wallet.getBalance();
    return ethers.formatEther(balance);
  }

  async sendTransaction(to: string, amount: string): Promise<ethers.TransactionResponse> {
    const tx = await this.wallet.sendTransaction({
      to,
      value: ethers.parseEther(amount),
    });
    return tx;
  }
}
