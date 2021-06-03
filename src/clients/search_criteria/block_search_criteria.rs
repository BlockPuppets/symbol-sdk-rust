use crate::account::{Address, PublicAccount};
use crate::{BlockOrderBy, H192};

/// Defines the params used to search blocks. With this criteria, you can sort and filter
/// block queries using rest.
///
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct BlockSearchCriteria {
    /// `PublicAccount` of the account signing the entity.
    /// Filter by `PublicAccount` of the account signing the entity.
    pub signer_public_key: Option<PublicAccount<H192>>,

    /// beneficiary `Address`.
    /// Filter by beneficiary Address.
    pub beneficiary_address: Option<Address<H192>>,

    /// Order by block id or height.
    /// Sort responses by the property set.
    pub order_by: Option<BlockOrderBy>,
}
