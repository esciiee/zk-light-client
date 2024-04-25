use ssz_rs::prelude::*;

pub fn u256_deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val: String = serde::Deserialize::deserialize(deserializer)?;
    let y = alloy_primitives::U256::from_str_radix(&val, 10).unwrap();
    let y_bytes = y.to_le_bytes();
    Ok(U256::from_bytes_le(y_bytes))
}

// pub fn header_deserialize<'de, D>(deserializer: D) -> Result<Header, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let header: LightClientHeader = serde::Deserialize::deserialize(deserializer)?;

//     Ok(match header {
//         LightClientHeader::Unwrapped(header) => header,
//         LightClientHeader::Wrapped(header) => header.beacon,
//     })
// }

// #[derive(serde::Deserialize, Debug)]
// #[serde(untagged)]
// enum LightClientHeader {
//     Unwrapped(Header),
//     Wrapped(Beacon),
// }

// #[derive(serde::Deserialize, Debug)]
// struct Beacon {
//     beacon: Header,
// }

macro_rules! superstruct_ssz {
    ($type:tt) => {
        impl ssz_rs::Merkleized for $type {
            fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
                match self {
                    $type::Bellatrix(inner) => inner.hash_tree_root(),
                    $type::Capella(inner) => inner.hash_tree_root(),
                    $type::Deneb(inner) => inner.hash_tree_root(),
                }
            }
        }

        impl ssz_rs::Sized for $type {
            fn is_variable_size() -> bool {
                true
            }

            fn size_hint() -> usize {
                0
            }
        }

        impl ssz_rs::Serialize for $type {
            fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
                match self {
                    $type::Bellatrix(inner) => inner.serialize(buffer),
                    $type::Capella(inner) => inner.serialize(buffer),
                    $type::Deneb(inner) => inner.serialize(buffer),
                }
            }
        }

        impl ssz_rs::Deserialize for $type {
            fn deserialize(_encoding: &[u8]) -> Result<Self, DeserializeError>
            where
                Self: Sized,
            {
                panic!("not implemented");
            }
        }

        impl ssz_rs::SimpleSerialize for $type {}
    };
}

/// this has to go after macro definition
pub(crate) use superstruct_ssz;
