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

/// An abstract transaction trait that serves as the base of all transaction types.
///
#[typetag::serde]
pub trait Transaction: Sync + Send
where
    Self: fmt::Debug,
{
    fn serializer(&self) -> Vec<u8>;

    fn to_embedded_transaction_builder(
        &self,
    ) -> Box<dyn buffer::embedded_transaction_helper::EmbeddedTransactionHelper>;
}
