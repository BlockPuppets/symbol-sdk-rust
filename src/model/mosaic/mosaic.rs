/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

use anyhow::{ensure, Result};

use crate::{ser_to_id, Uint64};

use super::UnresolvedMosaicId;

/// A `Mosaic` describes an instance of a mosaic definition.
/// Mosaics can be transferred by means of a transfer transaction.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mosaic {
    /// The mosaic id.
    /// This can either be of type `MosaicId` or `NamespaceId`.
    ///
    #[serde(serialize_with = "ser_to_id")]
    pub id: Box<dyn UnresolvedMosaicId + 'static>,
    /// The mosaic amount.
    /// The quantity is always given in smallest units for the mosaic
    /// i.e. if it has a divisibility of 3 the quantity is given in millis.
    ///
    pub amount: Uint64,
}

impl Mosaic {
    /// Determines the max decimal place to which the mosaic can be divided.
    /// The divisibility must be in the range of 0 and 6.
    ///
    pub const MAX_DIVISIBILITY: u8 = 6;

    pub const MIN_AMOUNT: u64 = 0;

    pub const MAX_AMOUNT_ABSOLUTE: u64 = 9_000_000_000_000_000;

    /// Create `Mosaic` with absolute amount.
    ///
    pub fn create<I: 'static + UnresolvedMosaicId>(id: I, amount: u64) -> Result<Self> {
        ensure!(
            amount <= Self::MAX_AMOUNT_ABSOLUTE,
            format!(
                "Invalid amount {}, the amount must be in the range of {} and {} atomic units.",
                amount,
                Self::MIN_AMOUNT,
                Self::MAX_AMOUNT_ABSOLUTE
            )
        );

        Ok(Self {
            id: Box::new(id),
            amount: Uint64::from(amount),
        })
    }

    /// Create `Mosaic` with relative amount.
    ///
    /// # Info
    ///
    /// Mosaic units in Symbol are defined as absolute amounts.
    /// To get an absolute amount, multiply the number of assets you want to send by 10 pow(divisibility).
    /// For example, if the mosaic had divisibility 2, to send 10 units (relative) you should define 1000 (absolute) instead.
    ///
    pub fn create_relative<I: 'static + UnresolvedMosaicId>(
        id: I,
        amount: u64,
        divisibility: u8,
    ) -> Result<Self> {
        ensure!(
            divisibility <= Self::MAX_DIVISIBILITY,
            format!(
                "Invalid divisibility {}, the divisibility must be in the range of 0 and 6.",
                divisibility
            )
        );

        let pow_divisibility = 10_u64.pow(divisibility as u32);

        let max_amount_relative = Self::MAX_AMOUNT_ABSOLUTE / pow_divisibility;
        ensure!(
            amount <= max_amount_relative,
            format!(
                "Invalid amount {}, the relative amount must be in the range of {} and {} units.",
                amount,
                Self::MIN_AMOUNT,
                max_amount_relative
            )
        );

        Self::create(id, amount * pow_divisibility)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }

    /// Create Builder object
    pub fn to_builder(&self) -> buffer::unresolved_mosaic_builder::UnresolvedMosaicBuilder {
        buffer::unresolved_mosaic_builder::UnresolvedMosaicBuilder::from_binary(&*self.to_vec())
    }
}

impl fmt::Display for Mosaic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "{")?;
        writeln!(f, "  \"id\": \"{}\",", self.id)?;
        writeln!(f, "  \"amount\": {}", *self.amount)?;
        writeln!(f, "{}", "}")
    }
}

#[cfg(test)]
mod tests {
    use crate::mosaic::{Mosaic, MosaicId};

    const LO_HI: [u32; 2] = [3646934825, 3576016193];

    #[test]
    fn test_should_create_with_absolute_amount() {
        let id = MosaicId::from(LO_HI);
        let mosaic = Mosaic::create(id, 1).unwrap();

        assert_eq!(mosaic.id.as_ref(), &id);
        assert_eq!(*mosaic.amount, 1);
    }

    #[test]
    fn test_should_create_with_relative_amount() {
        let id = MosaicId::from(LO_HI);
        let mosaic = Mosaic::create_relative(id, 1, 6).unwrap();

        assert_eq!(mosaic.id.as_ref(), &id);
        assert_eq!(*mosaic.amount, 1_000_000);
    }

    #[test]
    #[should_panic(
    expected = "Invalid divisibility 8, the divisibility must be in the range of 0 and 6."
    )]
    fn test_try_create_with_relative_should_return_panic() {
        let id = MosaicId::from(LO_HI);
        Mosaic::create_relative(id, 1, 8).unwrap();
    }
}
