use super::chain::*;
use super::id::*;
use super::signedtransaction::*;
use super::transaction::*;
use super::*;

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
            "in_block: {}; UXTO: {}; amount: {};",
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
    pub uxtos: Vec<UXTO>,
    pub total_credits: u128,
    pub rsa_pair: openssl::rsa::Rsa<openssl::pkey::Private>,
    pub id: Id,
}

impl Wallet {
    pub fn new() -> Self {
        let rsa_pair = Rsa::generate(1024).unwrap();
        let id = Id::new(&bs58::encode(rsa_pair.public_key_to_der().unwrap()).into_string());
        Self {
            uxtos: vec![],
            total_credits: 0,
            rsa_pair,
            id,
        }
    }

    pub fn read_wallet(&mut self, chain: &BlockChain) {
        let is_sender_of_tx = |id: &Id, tx: &SignedTransaction| id.id == tx.transaction.sender;

        let is_recipient_of_tx =
            |id: &Id, tx: &SignedTransaction| id.id == tx.transaction.recipient;

        let filter_txs_in_chain = |chain: &BlockChain,
                                   predicate: fn(&Id, &SignedTransaction) -> bool|
         -> Vec<(u128, SignedTransaction)> {
            chain
                .chain
                .iter()
                .flat_map(|block| {
                    block
                        .transactions
                        .iter()
                        .filter_map(|tx| {
                            if predicate(&self.id, tx) {
                                Some((block.index, tx.clone()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(u128, SignedTransaction)>>()
                })
                .collect()
        };

        let received_txs = filter_txs_in_chain(chain, is_recipient_of_tx);
        let spent_txs = filter_txs_in_chain(chain, is_sender_of_tx);

        // gather UXTOs => {recived - spent}
        self.uxtos = received_txs
            .iter()
            .filter_map(|(index, in_tx)| {
                let is_uxto = spent_txs
                    .iter()
                    .any(|(_, out_uxto)| out_uxto.transaction.intx == in_tx.hash());
                if !is_uxto {
                    Some(UXTO {
                        block_id: *index,
                        hash: in_tx.hash(),
                        amount: in_tx.transaction.amount,
                    })
                } else {
                    None
                }
            })
            .collect();

        self.total_credits = self.uxtos.iter().fold(0, |acc, uxto| acc + uxto.amount);
    }

    pub fn create_transaction(
        &mut self,
        recipient: &Id,
        amount: u128,
    ) -> Result<Vec<Transaction>, WalletOperationResult> {
        log::debug!(
            "##################### Creating transaction for {} coins #####################",
            amount
        );

        let mut sum: u128 = 0;
        let mut intxs = vec![];

        log::debug!("Gathering UXTOs:");

        for uxto in self.uxtos.iter() {
            if sum >= amount {
                break;
            }
            log::debug!("\tAdding UXTO: {}", uxto);
            intxs.push(uxto);
            sum += uxto.amount;
        }

        if sum < amount {
            return Err(WalletOperationResult::NotEnoughtCoinsError);
        }

        log::debug!("Gathered INTXs worth of {} coins", sum);

        assert!(sum >= amount);

        log::debug!("Preparing transactions:");
        let mut transfers = vec![];
        let mut processed_transfer = 0;
        for intx in intxs.iter() {
            let fraction_to_transfer = cmp::min(amount - processed_transfer, intx.amount);

            processed_transfer += fraction_to_transfer;

            let transaction = Transaction::new(
                intx.block_id,
                &intx.hash,
                &self.id.id,
                &recipient.id,
                fraction_to_transfer,
            );

            log::debug!("\tTransaction from UXTO: {}", transaction);

            transfers.push(transaction);

            let fraction_to_send_back = intx.amount - fraction_to_transfer;
            if fraction_to_send_back > 0 {
                let transfer_difference = Transaction::new(
                    intx.block_id,
                    &intx.hash,
                    &self.id.id,
                    &self.id.id,
                    fraction_to_send_back,
                );

                log::debug!("\tTransfer back from UXTO: {}", transfer_difference);
                transfers.push(transfer_difference);
            }
        }

        //remove used UXTOS from the wallet
        for transfer in &transfers {
            let used_uxto = &transfer.intx;
            self.total_credits = self.total_credits.saturating_sub(transfer.amount);
            if let Some(index) = self.uxtos.iter().position(|u| *used_uxto == u.hash) {
                self.uxtos.remove(index);
            }
        }

        assert_eq!(amount, processed_transfer);
        log::debug!("##################### Transaction created #####################");
        Ok(transfers)
    }

    pub fn sign_transaction(&self, tx: &Transaction) -> SignedTransaction {
        let mut signature = vec![0; self.rsa_pair.size() as usize];
        let _ = self
            .rsa_pair
            .private_encrypt(tx.hash().as_bytes(), &mut signature, Padding::PKCS1)
            .unwrap();

        SignedTransaction::new(tx.clone(), bs58::encode(signature).into_string())
    }

    pub fn sign_transactions(&self, transfers: Vec<Transaction>) -> Vec<SignedTransaction> {
        transfers
            .iter()
            .map(|tx| self.sign_transaction(tx))
            .collect()
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Wallet {}...", &self.id)?;
        if self.uxtos.is_empty() {
            return write!(f, "\tWallet is empty");
        }

        for uxto in &self.uxtos[..self.uxtos.len() - 1] {
            writeln!(f, "\t{}", uxto)?;
        }

        write!(f, "\t{}", self.uxtos.last().unwrap())
    }
}
