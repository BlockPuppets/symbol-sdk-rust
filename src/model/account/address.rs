/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{anyhow, ensure, Result};
use hex::ToHex;

use crate::{H256, hex_decode};
use crate::core::format::{
    decode_base32, encode_base32, is_valid_address, public_key_to_address, raw_prettify,
};
use crate::helpers::H192;
use crate::helpers::is_hex;
use crate::network::NetworkType;

/// The `Address` struct describes an Symbol address with its network.
///
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Address {
    /// The Symbol address in `H192`.
    pub address: H192,

    /// The Symbol network type.
    pub network_type: NetworkType,
}

impl Address {
    /// The length of the Symbol `Address` in base32 string.
    pub const LENGTH_IN_BASE32: usize = 39;

    pub const LENGTH_IN_DECODED: usize = std::mem::size_of::<H192>();

    pub const CHECKSUM_SIZE: usize = 3;

    /// Get the `Address` in an raw address string format.
    ///
    /// For example: TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q
    pub fn address_str(&self) -> String {
        encode_base32(self.address.as_bytes())
    }

    /// Converts `Address` String into a more readable/pretty format,
    /// a Symbol prettify address string looks like:
    ///
    /// * Before: TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q
    /// * After: TATNE7-Q5BITM-UTRRN6-IB4I7F-LSDRDW-ZA37JG-O5Q
    pub fn prettify(&self) -> String {
        raw_prettify(&self.address_str())
    }

    /// `Address` in the encoded format ex:
    ///
    /// * Before: 985328D6382688F9EF65EBCE38857910D8A64031B784BC17
    /// * After: TBJSRVRYE2EPT33F5PHDRBLZCDMKMQBRW6CLYFY
    pub fn address_encoded(&self) -> String {
        self.address.as_bytes().encode_hex_upper::<String>()
    }

    /// Creates an Symbol `Address` from a given public_key string.
    ///
    /// # Inputs
    ///
    /// * `public_key`: representing the hex publick key (String or &str).
    ///
    /// * `network_type`: The `NetworkType` of Sybol Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Address;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let public_key: &str = "2E834140FD66CF87B254A693A2C7862C819217B676D3943267156625E816EC6F";
    /// let address = Address::from_public_key(public_key, NetworkType::TestNet).unwrap();
    /// # println!("{}", address);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `Address` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_public_key(public_key: &str, network_type: NetworkType) -> Result<Self> {
        ensure!(is_hex(public_key), "public_key it's not hex.");

        let public_key_hash =
            H256::from_str(public_key).map_err(|e| anyhow!("public_key {}", e))?;

        let address_vec = public_key_to_address(public_key_hash, network_type);

        Ok(Self {
            address: H192::from_slice(address_vec.as_slice()),
            network_type,
        })
    }

    /// Creates an Symbol 'Address' from a given raw address string.
    ///
    /// # Info
    ///
    /// A Symbol raw address string looks like:
    /// * TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q or TATNE7-Q5BITM-UTRRN6-IB4I7F-LSDRDW-ZA37JG-O5Q.
    ///
    /// # Inputs
    ///
    /// * `raw_address`: an `S` representing address (String or &str).
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Address;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let raw_address: &str = "TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q";
    /// let address = Address::from_raw(raw_address).unwrap();
    /// # println!("{}", address);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `Address` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_raw<S: AsRef<str>>(raw_address: S) -> Result<Self> {
        let address_raw = raw_address.as_ref().trim().replace("-", "");
        ensure!(
            address_raw.len() == Self::LENGTH_IN_BASE32,
            "Address {} has to be {} characters long",
            address_raw,
            Self::LENGTH_IN_BASE32
        );

        let network_identifier = address_raw.to_uppercase().chars().next().unwrap();

        let network_type = NetworkType::try_from(network_identifier)?;

        let address = H192::from_base32(&address_raw);

        Ok(Self {
            address,
            network_type,
        })
    }

    /// Create an Symbol `Address` from the given hex string address
    ///
    /// A hex string address looks like: 908E2C873E8552039933562AFB74A193B48BDD300BEBDB93.
    ///
    /// # Inputs
    ///
    /// * `encoded`: an `S` representing address hex (String or &str).
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Address;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let encoded: &str = "908E2C873E8552039933562AFB74A193B48BDD300BEBDB93";
    /// let address = Address::from_encoded(encoded).unwrap();
    /// # println!("{}", address);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `Address` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_encoded<S: AsRef<str>>(encoded: S) -> Result<Self> {
        ensure!(is_hex(encoded.as_ref()), "encoded address it's not hex.");

        let address =
            H192::from_str(encoded.as_ref()).map_err(|e| anyhow!("encoded address {}", e))?;

        Self::from_raw(encode_base32(address.as_bytes()))
    }

    /// Determines the validity of an raw address string.
    ///
    /// # Inputs
    ///
    /// * `raw_address`: The raw address string. Expected format VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ.
    ///
    /// # Returns
    ///
    /// true if the raw address string is valid, false otherwise.
    ///
    pub fn is_valid_raw_address(raw_address: String) -> bool {
        if !['A', 'I', 'Q', 'Y'].contains(raw_address.chars().last().as_ref().unwrap()) {
            return false;
        };
        let mut address = H192::zero();
        decode_base32(address.as_mut(), &raw_address);
        is_valid_address(
            address.as_bytes(),
            Self::LENGTH_IN_DECODED,
            Self::CHECKSUM_SIZE,
        )
    }

    /// Determines the validity of an encoded address string.
    /// # Inputs
    ///
    /// * `encoded`: The encoded address string. Expected format: 6823BB7C3C089D996585466380EDBDC19D4959184893E38C.
    /// # Returns
    ///
    /// true if the encoded address string is valid, false otherwise.
    ///
    pub fn is_valid_encoded_address(encoded: String) -> bool {
        if !is_hex(&encoded) {
            return false;
        }
        is_valid_address(
            &hex_decode(&encoded),
            Self::LENGTH_IN_DECODED,
            Self::CHECKSUM_SIZE,
        )
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Account;
    use crate::account::Address;
    use crate::network::NetworkType;

    const PUBLIC_KEY: &str = "2E834140FD66CF87B254A693A2C7862C819217B676D3943267156625E816EC6F";

    #[test]
    fn test_should_create_given_public_key_with_network_type_private_test() {
        let address = Address::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
        assert_eq!(
            address.address_str(),
            "VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ"
        );
        assert_eq!(address.network_type, NetworkType::PrivateTest);
    }

    #[test]
    fn test_should_print_the_address_in_pretty_format() {
        let address = Address::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
        assert_eq!(
            address.prettify(),
            "VATNE7-Q5BITM-UTRRN6-IB4I7F-LSDRDW-ZA35C4-KNQ"
        );
    }

    #[test]
    fn test_should_create_given_public_key_with_network_type_private() {
        let address = Address::from_public_key(PUBLIC_KEY, NetworkType::Private).unwrap();
        assert_eq!(
            address.address_str(),
            "PATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35OETNI"
        );
        assert_eq!(address.network_type, NetworkType::Private);
    }

    #[test]
    fn test_should_create_given_public_key_with_network_type_main_net() {
        let address = Address::from_public_key(PUBLIC_KEY, NetworkType::MainNet).unwrap();
        assert_eq!(
            address.address_str(),
            "NATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA34SQ33Y"
        );
        assert_eq!(address.network_type, NetworkType::MainNet);
    }

    #[test]
    fn test_should_create_given_public_key_with_network_type_test_net() {
        let address = Address::from_public_key(PUBLIC_KEY, NetworkType::TestNet).unwrap();
        assert_eq!(
            address.address_str(),
            "TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q"
        );
        assert_eq!(address.network_type, NetworkType::TestNet);
    }

    #[test]
    fn test_should_create_given_vatne7q5bitmutrrn6ib4i7flsdrdwza35c4knq() {
        let address = Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap();
        assert_eq!(address.network_type, NetworkType::PrivateTest);
    }

    #[test]
    fn test_should_create_given_patne7q5bitmutrrn6ib4i7flsdrdwza35oetni() {
        let address = Address::from_raw("PATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35OETNI").unwrap();
        assert_eq!(address.network_type, NetworkType::Private);
    }

    #[test]
    fn test_should_create_given_natne7q5bitmutrrn6ib4i7flsdrdwza34sq33y() {
        let address = Address::from_raw("NATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA34SQ33Y").unwrap();
        assert_eq!(address.network_type, NetworkType::MainNet);
    }

    #[test]
    fn test_should_create_given_tatne7q5bitmutrrn6ib4i7flsdrdwza37jgo5q() {
        let address = Address::from_raw("TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q").unwrap();
        assert_eq!(address.network_type, NetworkType::TestNet);
    }

    #[test]
    fn test_should_create_given_vatne7_q5bitm_utrrn6_ib4i7f_lsdrdw_za35c4_knq() {
        let address = Address::from_raw("VATNE7-Q5BITM-UTRRN6-IB4I7F-LSDRDW-ZA35C4-KNQ").unwrap();
        assert_eq!(address.network_type, NetworkType::PrivateTest);
        assert_eq!(
            address.prettify(),
            "VATNE7-Q5BITM-UTRRN6-IB4I7F-LSDRDW-ZA35C4-KNQ"
        );
    }

    #[test]
    #[should_panic(expected = "Address Network unsupported")]
    fn test_should_panic_when_the_address_contain_an_invalid_network_identifier() {
        Address::from_raw("ZCTVW23D2MN5VE4AQ4TZIDZENGNOZXPRPSDRSFR").unwrap();
    }

    #[test]
    #[should_panic(
    expected = "Address ZCTVW234AQ4TZIDZENGNOZXPRPSDRSFRF has to be 39 characters long"
    )]
    fn test_should_panic_when_the_address_is_not_valid_in_length() {
        Address::from_raw("ZCTVW234AQ4TZIDZENGNOZXPRPSDRSFRF").unwrap();
    }

    #[test]
    fn test_should_turn_a_lowercase_address_to_uppercase() {
        let address = Address::from_raw("tatne7q5bitmutrrn6ib4i7flsdrdwza37jgo5q").unwrap();
        assert_eq!(
            address.address_str(),
            "TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q"
        );
    }

    #[test]
    fn test_should_equal_addresses() {
        let address = Address::from_raw("TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q").unwrap();
        let compare_address = Address::from_raw("TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q").unwrap();

        assert_eq!(address, compare_address);
    }

    #[test]
    fn test_should_not_equal_addresses() {
        let address = Address::from_raw("TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q").unwrap();
        let compare_address = Address::from_raw("TDR6EW2WBHJQDYMNGFX2UBZHMMZC5PGL2YBO3KA").unwrap();

        assert_ne!(address, compare_address);
    }

    #[test]
    fn test_should_creates_from_an_encoded_value() {
        let encoded = "917E7E29A01014C2F3000000000000000000000000000000";
        let address = Address::from_encoded(encoded).unwrap();
        assert_eq!(address.address_encoded(), encoded);
    }

    #[cfg(test)]
    mod tests_valid_raw_address {
        use super::*;

        #[test]
        fn test_returns_true_for_valid_address_when_generated() {
            assert!(Address::is_valid_raw_address(
                Account::random(NetworkType::PrivateTest).address_str()
            ));
            assert!(Address::is_valid_raw_address(
                Account::random(NetworkType::MainNet).address_str()
            ));
            assert!(Address::is_valid_raw_address(
                Account::random(NetworkType::Private).address_str()
            ));
            assert!(Address::is_valid_raw_address(
                Account::random(NetworkType::TestNet).address_str()
            ));
        }

        #[test]
        fn test_returns_true_for_valid_address() {
            let raw_address = "VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ";
            assert!(Address::is_valid_raw_address(raw_address.to_string()));
        }

        #[test]
        fn test_returns_false_for_address_with_invalid_checksum() {
            let raw_address = "SATNE7Q5BITMUTRRN6YB4I7FLSDRDWZA34I2PMQ";
            assert_eq!(
                Address::is_valid_raw_address(raw_address.to_string()),
                false
            );
        }

        #[test]
        fn test_returns_false_for_address_with_invalid_hash() {
            let raw_address = "SATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA34I2PQQ";
            assert_eq!(
                Address::is_valid_raw_address(raw_address.to_string()),
                false
            );
        }

        #[test]
        fn test_returns_false_for_address_with_invalid_prefix() {
            let raw_address = "AATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA34I2PMQ";
            assert_eq!(
                Address::is_valid_raw_address(raw_address.to_string()),
                false
            );
        }

        #[test]
        fn test_returns_true_if_last_char_in_a_or_i_or_q_or_y() {
            const RAW_ADDRESS: [&str; 4] = [
                "NAR3W7B4BCOZSZMFIZRYB3N5YGOUSWIYJCJ6HDA",
                "TDZ4373ASEGJ7S7GQTKF26TIIMC7HK5EWEPHRSI",
                "PDZ4373ASEGJ7S7GQTKF26TIIMC7HK5EWELJG3Y",
                "MCOVTFVVDZGNURZFU4IJLJR37X5TXNWMTTARXZQ",
            ];

            for address in &RAW_ADDRESS {
                assert!(Address::is_valid_raw_address(address.to_string()));
            }
        }
    }

    #[cfg(test)]
    mod tests_valid_encoded_address {
        use super::*;

        #[test]
        fn test_returns_true_for_valid_address_when_generated() {
            assert!(Address::is_valid_encoded_address(
                Account::random(NetworkType::PrivateTest)
                    .public_account
                    .address
                    .address_encoded()
            ));
            assert!(Address::is_valid_encoded_address(
                Account::random(NetworkType::MainNet)
                    .public_account
                    .address
                    .address_encoded()
            ));
            assert!(Address::is_valid_encoded_address(
                Account::random(NetworkType::Private)
                    .public_account
                    .address
                    .address_encoded()
            ));
            assert!(Address::is_valid_encoded_address(
                Account::random(NetworkType::TestNet)
                    .public_account
                    .address
                    .address_encoded()
            ));
        }

        #[test]
        fn test_returns_true_for_valid_encoded_address() {
            let encoded = "6823BB7C3C089D996585466380EDBDC19D4959184893E38C";
            assert!(Address::is_valid_encoded_address(encoded.to_string()));
        }

        #[test]
        fn test_returns_false_for_invalid_hex_encoded_address() {
            let encoded = "Z823BB7C3C089D996585466380EDBDC19D4959184893E38C";
            assert_eq!(
                Address::is_valid_encoded_address(encoded.to_string()),
                false
            );
        }

        #[test]
        fn test_returns_false_for_invalid_encoded_address() {
            let encoded = "6823BB7C3C089D996585466380EDBDC19D4959184893E38D";
            assert_eq!(
                Address::is_valid_encoded_address(encoded.to_string()),
                false
            );
        }

        #[test]
        fn test_returns_false_for_encoded_address_with_wrong_length() {
            let encoded = "6823BB7C3C089D996585466380EDBDC19D4959184893E38CEE";
            assert_eq!(
                Address::is_valid_encoded_address(encoded.to_string()),
                false
            );
        }

        #[test]
        fn test_adding_leading_or_trailing_white_space_invalidates_encoded_address() {
            let encoded = "6823BB7C3C089D996585466380EDBDC19D4959184893E38C";
            assert_eq!(
                Address::is_valid_encoded_address(format!(" \t {}", encoded)),
                false
            );
            assert_eq!(
                Address::is_valid_encoded_address(format!("{} \t ", encoded)),
                false
            );
            assert_eq!(
                Address::is_valid_encoded_address(format!(" \t {} \t ", encoded)),
                false
            );
        }
    }
}
