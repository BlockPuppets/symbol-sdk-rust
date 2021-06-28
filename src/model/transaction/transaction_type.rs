use std::convert::TryFrom;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[repr(u16)]
pub enum TransactionType {
    /// Reserved entity type.
    Reserved = 0,

    /// Transfer Transaction transaction type.
    /// Decimal value = 16724
    /// Hex value = 0x4154
    ///
    Transfer = 0x4154,

    /// Register namespace transaction type.
    /// Decimal value = 16718
    /// Hex value = 0x414e
    ///
    RegisterNamespace = 0x414e,

    /// Address alias transaction type.
    /// Decimal value = 16974
    /// Hex value = 0x424E
    ///
    AddressAlias = 0x424E,

    /// Mosaic alias transaction type.
    /// Decimal value = 17230
    /// Hex value = 0x434E
    ///
    MosaicAlias = 0x434E,

    /// Mosaic definition transaction type.
    /// Decimal value = 16717
    /// Hex value = 0x414d
    ///
    MosaicDefinition = 0x414d,

    /// Mosaic supply change transaction.
    /// Decimal value = 16973
    /// Hex value = 0x424d
    ///
    MosaicSupplyChange = 0x424d,

    /// Modify multisig account transaction type.
    /// Decimal value = 16725
    /// Hex value = 0x4155
    ///
    MultisigAccountModify = 0x4155,

    /// Aggregate complete transaction type.
    /// Decimal value = 16705
    /// Hex value = 0x4141
    ///
    AggregateComplete = 0x4141,

    /// Aggregate bonded transaction type.
    /// Decimal value = 16961
    /// Hex value = 0x4241
    ///
    AggregateBonded = 0x4241,

    /// Lock transaction type.
    /// Decimal value = 16712
    /// Hex value = 0x4148
    ///
    Hash_Lock = 0x4148,

    /// Secret Lock Transaction type.
    /// Decimal value = 16722
    /// Hex value = 0x4152
    ///
    SecretLock = 0x4152,

    /// Secret Proof transaction type.
    /// Decimal value = 16978
    /// Hex value = 0x4252
    ///
    SecretProof = 0x4252,

    /// Account restriction address transaction type.
    /// Decimal value = 16720
    /// Hex value = 0x4150
    ///
    AccountRestrictionAddress = 0x4150,

    /// Account restriction mosaic transaction type.
    /// Decimal value = 16976
    /// Hex value = 0x4250
    ///
    AccountRestrictionMosaic = 0x4250,

    /// Account restriction operation transaction type.
    /// Decimal value = 17232
    /// Hex value = 0x4350
    ///
    AccountRestrictionOperation = 0x4350,

    /// Mosaic address restriction type.
    /// Decimal value = 16977
    /// Hex value = 0x4251
    ///
    MosaicAddressRestriction = 0x4251,

    /// Mosaic global restriction type.
    /// Decimal value = 16721
    /// Hex value = 0x4151
    ///
    MosaicGlobalRestriction = 0x4151,

    /// Account metadata transaction type.
    /// Decimal value = 16708
    /// Hex value = 0x4144
    ///
    AccountMetadata = 0x4144,

    /// Mosaic metadata transaction type.
    /// Decimal value = 16964
    /// Hex value = 0x4244
    ///
    MosaicMetadata = 0x4244,

    /// Namespace metadata transaction type.
    /// Decimal value = 17220
    /// Hex value = 0x4344
    ///
    NamespaceMetadata = 0x4344,

    /// Link account transaction type.
    /// Decimal value = 16716
    /// Hex value = 0x414C
    ///
    AccountKeyLink = 0x414C,

    /// Link vrf key transaction type.
    /// Decimal value = 16963
    /// Hex value = 0x4243
    ///
    VrfKeyLink = 0x4243,

    /// Link voting key transaction type.
    /// Decimal value = 16707
    /// Hex value = 0x4143
    ///
    VotingKeyLink = 0x4143,

    /// Link node key transaction type.
    /// Decimal value = 16972
    /// Hex value = 0x424C
    ///
    NodeKeyLink = 0x424C,
}

impl TransactionType {
    const UNKNOWN_TRANSACTION_TYPE: &'static str = "Unknown transaction type";

    pub fn value(self) -> u16 {
        self as u16
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        self.value().to_le_bytes()
    }
}

/// Returns a 'TransactionType' for the given u16 value.
///
/// Throws an Err UNKNOWN_TRANSACTION_TYPE when the type is unknown.
impl TryFrom<u16> for TransactionType {
    type Error = anyhow::Error;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        use TransactionType::*;
        match v {
            x if x == AccountRestrictionAddress as u16 => Ok(AccountRestrictionAddress),
            x if x == AccountKeyLink as u16 => Ok(AccountKeyLink),
            x if x == AccountMetadata as u16 => Ok(AccountMetadata),
            x if x == AccountRestrictionMosaic as u16 => Ok(AccountRestrictionMosaic),
            x if x == AccountRestrictionOperation as u16 => Ok(AccountRestrictionOperation),
            x if x == AddressAlias as u16 => Ok(AddressAlias),
            x if x == AggregateBonded as u16 => Ok(AggregateBonded),
            x if x == AggregateComplete as u16 => Ok(AggregateComplete),
            x if x == Hash_Lock as u16 => Ok(Hash_Lock),
            x if x == MosaicAddressRestriction as u16 => Ok(MosaicAddressRestriction),
            x if x == MosaicAlias as u16 => Ok(MosaicAlias),
            x if x == MosaicDefinition as u16 => Ok(MosaicDefinition),
            x if x == MosaicGlobalRestriction as u16 => Ok(MosaicGlobalRestriction),
            x if x == MosaicMetadata as u16 => Ok(MosaicMetadata),
            x if x == MosaicSupplyChange as u16 => Ok(MosaicSupplyChange),
            x if x == MultisigAccountModify as u16 => Ok(MultisigAccountModify),
            x if x == NamespaceMetadata as u16 => Ok(NamespaceMetadata),
            x if x == RegisterNamespace as u16 => Ok(RegisterNamespace),
            x if x == NodeKeyLink as u16 => Ok(NodeKeyLink),
            x if x == SecretLock as u16 => Ok(SecretLock),
            x if x == SecretProof as u16 => Ok(SecretProof),
            x if x == Transfer as u16 => Ok(Transfer),
            x if x == VotingKeyLink as u16 => Ok(VotingKeyLink),
            x if x == VrfKeyLink as u16 => Ok(VrfKeyLink),
            _ => Err(anyhow::anyhow!(Self::UNKNOWN_TRANSACTION_TYPE)),
        }
    }
}

/// Returns a 'TransactionType' for the given [u8; 2] value.
///
/// Throws an Err UNKNOWN_TRANSACTION_TYPE when the type is unknown.
impl TryFrom<[u8; 2]> for TransactionType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        Self::try_from(u16::from_le_bytes(value))
    }
}

impl From<TransactionType> for u16 {
    fn from(value: TransactionType) -> Self {
        value as u16
    }
}

impl From<TransactionType> for [u8; 2] {
    fn from(value: TransactionType) -> Self {
        value.to_bytes()
    }
}

/// Creates `TransactionType` with the default parameters.
///
impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::Transfer
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::TransactionType;

    #[test]
    fn test_should_match_the_specification() {
        assert_eq!(TransactionType::AccountRestrictionAddress as u16, 0x4150);
        assert_eq!(TransactionType::AccountKeyLink as u16, 0x414C);
        assert_eq!(TransactionType::AccountMetadata as u16, 0x4144);
        assert_eq!(TransactionType::AccountRestrictionMosaic as u16, 0x4250);
        assert_eq!(TransactionType::AccountRestrictionOperation as u16, 0x4350);
        assert_eq!(TransactionType::AddressAlias as u16, 0x424E);
        assert_eq!(TransactionType::AggregateBonded as u16, 0x4241);
        assert_eq!(TransactionType::AggregateComplete as u16, 0x4141);
        assert_eq!(TransactionType::Hash_Lock as u16, 0x4148);
        assert_eq!(TransactionType::MosaicAddressRestriction as u16, 0x4251);
        assert_eq!(TransactionType::MosaicAlias as u16, 0x434E);
        assert_eq!(TransactionType::MosaicDefinition as u16, 0x414d);
        assert_eq!(TransactionType::MosaicGlobalRestriction as u16, 0x4151);
        assert_eq!(TransactionType::MosaicMetadata as u16, 0x4244);
        assert_eq!(TransactionType::MosaicSupplyChange as u16, 0x424d);
        assert_eq!(TransactionType::MultisigAccountModify as u16, 0x4155);
        assert_eq!(TransactionType::NamespaceMetadata as u16, 0x4344);
        assert_eq!(TransactionType::RegisterNamespace as u16, 0x414e);
        assert_eq!(TransactionType::NodeKeyLink as u16, 0x424C);
        assert_eq!(TransactionType::SecretLock as u16, 0x4152);
        assert_eq!(TransactionType::SecretProof as u16, 0x4252);
        assert_eq!(TransactionType::Transfer as u16, 0x4154);
        assert_eq!(TransactionType::VotingKeyLink as u16, 0x4143);
        assert_eq!(TransactionType::VrfKeyLink as u16, 0x4243);
    }
}
