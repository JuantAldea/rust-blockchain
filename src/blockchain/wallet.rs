use super::block::*;
use super::chain::*;
use super::transaction::*;
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
            "in_block: {}: {}: {}",
            self.block_id, self.hash, self.amount
        )
    }
}
#[derive(Debug, Clone)]
pub struct Wallet {
    pub unexpend: Vec<UXTO>,
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
            for (_, out_uxto) in &ouput_uxtos {
                if out_uxto.input_uxto_hash == in_uxto.transaction_hash {
                    expend = true;
                    break;
                }
            }

            if !expend {
                total_credits += in_uxto.amount;
                unexpend.push(UXTO {
                    block_id: index_in,
                    hash: in_uxto.transaction_hash,
                    amount: in_uxto.amount,
                });
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
            let uxto: &UXTO = uxtos_iter.next().unwrap();
            uxtos.push(uxto);
            sum += uxto.amount;
            //println!("Adding uxto: {}: {} : {}", uxto.0, uxto.1, uxto.2)
        }

        assert!(sum >= amount);

        let mut transfers = vec![];
        let mut processed_transfer = 0;
        for uxto in uxtos.iter() {
            let fraction_to_transfer = cmp::min(amount - processed_transfer, uxto.amount);

            processed_transfer += fraction_to_transfer;
            transfers.push(Transaction::new(
                uxto.block_id,
                &uxto.hash,
                self.id,
                id,
                fraction_to_transfer,
            ));

            let fraction_to_send_back = uxto.amount - fraction_to_transfer;
            if fraction_to_send_back > 0 {
                transfers.push(Transaction::new(
                    uxto.block_id,
                    &uxto.hash,
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

        for uxto in &self.unexpend[..self.unexpend.len() - 1] {
            writeln!(f, "{}", uxto)?;
        }

        write!(f, "{}", self.unexpend.last().unwrap())
    }
}
