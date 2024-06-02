use rust_decimal::prelude::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum AssetEnum {
    Btc,
    Usdt,
}

#[derive(Debug)]
pub struct Asset {
    available: Decimal,
    frozen: Decimal,
}

impl Asset {
    pub fn default() -> Self {
        Self {
            available: Decimal::ZERO,
            frozen: Decimal::ZERO,
        }
    }

    pub fn new(available: Decimal, frozen: Decimal) -> Self {
        Self { available, frozen }
    }

    pub fn get_available(&self) -> Decimal {
        self.available
    }

    pub fn get_frozen(&self) -> Decimal {
        self.frozen
    }

    pub fn add_available(&mut self, amount: Decimal) {
        self.available += amount;
    }

    pub fn add_frozen(&mut self, amount: Decimal) {
        self.frozen += amount;
    }
}
