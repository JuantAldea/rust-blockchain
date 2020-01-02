pub mod block;
pub mod chain;
pub mod id;
pub mod signedtransaction;
pub mod transaction;
pub mod wallet;

pub trait Hashable {
    fn hash(&self) -> String;
}

#[cfg(test)]
mod tests;
