use super::block::*;
use super::chain::*;
use super::transaction::*;
use std::cmp;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub unexpend: Vec<(u128, Transaction)>,
    pub total_credits: u128,
    pub id: u128,
}

impl Wallet {
    pub fn new(chain: &BlockChain, id: u128) -> Self {
        let mut input_uxtos = vec![];
        let mut ouput_uxtos = vec![];
        for block in &chain.chain {
            for transaction in &block.transactions {
                if transaction.recipient == id {
                    input_uxtos.push((block.index, transaction.clone()));
                }

                if transaction.sender == id {
                    ouput_uxtos.push((block.index, transaction.clone()));
                }
            }
        }

        let mut unexpend = vec![];
        let mut total_credits = 0;
        for (index_in, in_uxto) in input_uxtos {
            let mut expend = false;
            for (index_out, out_uxto) in &ouput_uxtos {
                if out_uxto.input_uxto_hash == in_uxto.transaction_hash {
                    expend = true;
                    break;
                }
            }
            if !expend {
                total_credits += in_uxto.amount;
                unexpend.push((index_in, in_uxto.clone()));
            }
        }

        Wallet {
            unexpend,
            total_credits,
            id,
        }
    }

    pub fn create_transaction(&self, id: u128, amount: u128) -> Option<Block> {
        let mut sum: u128 = 0;
        let mut uxtos = vec![];
        let mut uxtos_iter = self.unexpend.iter();

        if sum > self.total_credits {
            return None;
        }

        while sum < amount {
            let uxto: &(u128, Transaction) = uxtos_iter.next().unwrap();
            uxtos.push(uxto);
            sum += uxto.1.amount;
            println!("Adding uxto: {}: {}", uxto.0, uxto.1)
        }

        assert!(sum >= amount);

        let mut transfers = vec![];
        let mut processed_transfer = 0;
        for (block_id, transaction) in uxtos.iter() {
            let fraction_to_transfer = cmp::min(amount - processed_transfer, transaction.amount);
            let fraction_to_send_back = transaction.amount - fraction_to_transfer;
            processed_transfer += fraction_to_transfer;
            transfers.push(Transaction::new(
                *block_id,
                &transaction.transaction_hash,
                self.id,
                id,
                fraction_to_transfer,
            ));

            if fraction_to_send_back > 0 {
                transfers.push(Transaction::new(
                    *block_id,
                    &transaction.transaction_hash,
                    self.id,
                    self.id,
                    fraction_to_send_back,
                ));
            }
        }

        assert_eq!(amount, processed_transfer);

        Some(Block::new(transfers))
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.unexpend.is_empty() {
            return writeln!(f, "Wallet is empty");
        }

        for (i, transaction) in &self.unexpend[..self.unexpend.len() - 1] {
            writeln!(f, "in_block: {}: {} ", i, transaction)?;
        }

        write!(
            f,
            "in_block: {}: {} ",
            self.unexpend.last().unwrap().0,
            self.unexpend.last().unwrap().1
        )
    }
}
