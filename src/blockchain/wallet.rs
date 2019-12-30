use super::block::*;
use super::chain::*;
use super::id::*;
use super::transaction::*;
use openssl::rsa::{Padding, Rsa};
use std::cmp;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UXTO {
    pub block_id: u128,
    pub hash: String,
    pub amount: u128,
}

impl fmt::Display for UXTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "in_block: {}; uxto: {}; amount: {};",
            self.block_id, self.hash, self.amount
        )
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum WalletOperationResult {
    ResultOk,
    NotEnoughtCoinsError,
}

#[derive(Debug, Clone)]
pub struct Wallet {
    pub unexpend: Vec<UXTO>,
    pub total_credits: u128,
    pub rsa_pair: openssl::rsa::Rsa<openssl::pkey::Private>,
    pub id: Id,
}

impl Wallet {
    pub fn new() -> Self {
        let rsa_pair = Rsa::generate(1024).unwrap();
        let id = Id::new(&bs58::encode(rsa_pair.public_key_to_der().unwrap()).into_string());
        Wallet {
            unexpend: vec![],
            total_credits: 0,
            rsa_pair,
            id,
        }
    }

    pub fn read_wallet(&mut self, chain: &BlockChain) {
        let mut input_uxtos = vec![];
        let mut ouput_uxtos = vec![];
        for block in &chain.chain {
            for transaction in &block.transactions {
                // Gather our earnings
                if transaction.transaction.recipient == self.id.id {
                    input_uxtos.push((block.index, transaction.clone()));
                }

                // Gather our expenses
                if transaction.transaction.sender == self.id.id {
                    ouput_uxtos.push((block.index, transaction.clone()));
                }
            }
        }

        // gather our unexpended earnings
        self.unexpend = vec![];
        self.total_credits = 0;

        for (index_in, in_uxto) in input_uxtos {
            let mut expend = false;
            for (_, out_uxto) in &ouput_uxtos {
                if out_uxto.transaction.input_uxto_hash == in_uxto.uxto_hash() {
                    expend = true;
                    break;
                }
            }

            if !expend {
                self.total_credits += in_uxto.transaction.amount;
                self.unexpend.push(UXTO {
                    block_id: index_in,
                    hash: in_uxto.uxto_hash(),
                    amount: in_uxto.transaction.amount,
                });
            }
        }
    }

    pub fn create_transaction(
        &self,
        recipient: &Id,
        amount: u128,
    ) -> Result<Vec<Transaction>, WalletOperationResult> {
        log::debug!(
            "##################### Creating transaction for {} coins #####################",
            amount
        );
        let mut sum: u128 = 0;
        let mut uxtos = vec![];
        let mut uxtos_iter = self.unexpend.iter();

        if sum > self.total_credits {
            return Err(WalletOperationResult::NotEnoughtCoinsError);
        }

        log::debug!("Gathering enough UXTOS:");
        while sum < amount {
            let uxto: &UXTO = uxtos_iter.next().unwrap();
            uxtos.push(uxto);
            sum += uxto.amount;
            log::debug!("\tAdding uxto: {}", uxto);
        }

        log::debug!("Gathered UXTOS worth of {} coins", sum);

        assert!(sum >= amount);

        log::debug!("Preparing UXTOs transactions:");
        let mut transfers = vec![];
        let mut processed_transfer = 0;
        for uxto in uxtos.iter() {
            let fraction_to_transfer = cmp::min(amount - processed_transfer, uxto.amount);
            processed_transfer += fraction_to_transfer;
            let transaction = Transaction::new(
                uxto.block_id,
                &uxto.hash,
                &self.id.id,
                &recipient.id,
                fraction_to_transfer,
            );

            log::debug!("\tTransaction from UXTO: {}", transaction);

            transfers.push(transaction);

            let fraction_to_send_back = uxto.amount - fraction_to_transfer;
            if fraction_to_send_back > 0 {
                let transfer_difference = Transaction::new(
                    uxto.block_id,
                    &uxto.hash,
                    &self.id.id,
                    &self.id.id,
                    fraction_to_send_back,
                );

                log::debug!("\tTransfer back from UXTO: {}", transfer_difference);
                transfers.push(transfer_difference);
            }
        }

        assert_eq!(amount, processed_transfer);
        log::debug!("##################### Transaction created #####################");
        Ok(transfers)
    }

    pub fn sign_transaction(&self, tx: &Transaction) -> SignedTransaction {
        let mut stx = SignedTransaction::new(tx);
        let mut signature = vec![0; self.rsa_pair.size() as usize];

        let _ = self
            .rsa_pair
            .private_encrypt(
                stx.hash_for_signature().as_bytes(),
                &mut signature,
                Padding::PKCS1,
            )
            .unwrap();
        stx.signature = bs58::encode(signature).into_string();
        stx
    }

    pub fn sign_transactions(&self, transfers: Vec<Transaction>) -> Block {
        let signed_transfers = transfers.iter().map(|tx| self.sign_transaction(tx));
        Block::new(signed_transfers.collect())
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Wallet {}...", &self.id)?;
        if self.unexpend.is_empty() {
            return write!(f, "\tWallet is empty");
        }

        for uxto in &self.unexpend[..self.unexpend.len() - 1] {
            writeln!(f, "\t{}", uxto)?;
        }

        write!(f, "\t{}", self.unexpend.last().unwrap())
    }
}
