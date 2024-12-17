use anyhow::{Result, Context};
use ethers::{
    prelude::*,
    signers::{LocalWallet, Signer},
    types::{TransactionRequest, U256},
};
use std::sync::Arc;
use tracing::{info, warn};

pub struct WalletManager {
    wallet: LocalWallet,
    provider: Arc<Provider<Http>>,
}

impl WalletManager {
    pub async fn new(private_key: &str, rpc_url: &str) -> Result<Self> {
        let wallet = private_key.parse::<LocalWallet>()
            .context("Failed to parse private key")?;
        
        let provider = Provider::<Http>::try_from(rpc_url)
            .context("Failed to create provider")?;
        let provider = Arc::new(provider);

        Ok(Self {
            wallet,
            provider,
        })
    }

    pub async fn get_balance(&self) -> Result<U256> {
        let address = self.wallet.address();
        let balance = self.provider.get_balance(address, None).await
            .context("Failed to get balance")?;
        
        Ok(balance)
    }

    pub async fn send_tip(&self, to_address: &str, amount: U256) -> Result<H256> {
        let to_address: H160 = to_address.parse()
            .context("Invalid recipient address")?;

        let from_address = self.wallet.address();
        
        // Check if we have enough balance
        let balance = self.get_balance().await?;
        if balance <= amount {
            warn!("Insufficient balance for tip");
            return Err(anyhow::anyhow!("Insufficient balance"));
        }

        let nonce = self.provider
            .get_transaction_count(from_address, None)
            .await
            .context("Failed to get nonce")?;

        let gas_price = self.provider
            .get_gas_price()
            .await
            .context("Failed to get gas price")?;

        let tx = TransactionRequest::new()
            .to(to_address)
            .from(from_address)
            .value(amount)
            .nonce(nonce)
            .gas_price(gas_price);

        let signature = self.wallet
            .sign_transaction(&tx.into())
            .await
            .context("Failed to sign transaction")?;

        let tx_hash = self.provider
            .send_raw_transaction(signature)
            .await
            .context("Failed to send transaction")?;

        info!("Sent tip with transaction hash: {:?}", tx_hash);
        
        Ok(tx_hash)
    }

    pub fn get_address(&self) -> H160 {
        self.wallet.address()
    }
}
