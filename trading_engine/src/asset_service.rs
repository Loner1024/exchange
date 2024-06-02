use crate::asset::{Asset, AssetEnum};
use crate::transfer::TransferType;
use anyhow::{anyhow, Result};
use dashmap::DashMap;
use rust_decimal::Decimal;
use std::cell::RefCell;
use std::rc::Rc;

struct AssetService {
    user_assets: DashMap<usize, DashMap<AssetEnum, Rc<RefCell<Asset>>>>,
}

impl AssetService {
    pub fn new() -> Self {
        Self {
            user_assets: DashMap::new(),
        }
    }

    fn get_asset(&self, user_id: usize, asset_id: AssetEnum) -> Rc<RefCell<Asset>> {
        self.user_assets
            .entry(user_id)
            .or_default()
            .value()
            .entry(asset_id)
            .or_insert(Rc::new(RefCell::new(Asset::default())))
            .value()
            .clone()
    }

    pub fn try_transfer(
        &self,
        transfer_type: TransferType,
        from_user: usize,
        to_user: usize,
        asset: AssetEnum,
        amount: Decimal,
        check_balance: bool,
    ) -> Result<()> {
        if amount.is_zero() {
            return Ok(());
        }
        if amount.is_sign_negative() {
            return Err(anyhow!("Amount must be positive"));
        }

        let from_asset = self.get_asset(from_user, asset);
        let to_asset = self.get_asset(to_user, asset);
        return match transfer_type {
            TransferType::AvailableToAvailable => {
                if check_balance && from_asset.borrow().get_available() < amount {
                    Err(anyhow!("Insufficient balance"))
                } else {
                    from_asset.borrow_mut().add_available(-amount);
                    to_asset.borrow_mut().add_available(amount);
                    Ok(())
                }
            }
            TransferType::AvailableToFrozen => {
                if check_balance && from_asset.borrow().get_available() < amount {
                    Err(anyhow!("Insufficient balance"))
                } else {
                    from_asset.borrow_mut().add_available(-amount);
                    to_asset.borrow_mut().add_frozen(amount);
                    Ok(())
                }
            }
            TransferType::FrozenToAvailable => {
                if check_balance && from_asset.borrow().get_frozen() < amount {
                    Err(anyhow!("Insufficient balance"))
                } else {
                    from_asset.borrow_mut().add_frozen(-amount);
                    to_asset.borrow_mut().add_available(amount);
                    Ok(())
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_try_transfer() {
        let asset_service = AssetService::new();
        let user1 = 1;
        let user2 = 2;
        let asset = AssetEnum::Usdt;
        let amount = dec!(100);

        asset_service
            .try_transfer(
                TransferType::AvailableToAvailable,
                user1,
                user2,
                asset,
                amount,
                false,
            )
            .unwrap();
        assert_eq!(
            asset_service
                .get_asset(user1, asset)
                .borrow()
                .get_available(),
            dec!(-100)
        );
        assert_eq!(
            asset_service
                .get_asset(user2, asset)
                .borrow()
                .get_available(),
            dec!(100)
        );
    }
}
