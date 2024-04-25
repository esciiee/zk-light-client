use ssz_rs::prelude::*;
use superstruct::superstruct;
use crate::types::*;


pub mod types;

#[superstruct(
    variants(Bellatrix, Capella, Deneb),
    variant_attributes(
        derive(serde::Deserialize, Clone, Debug, SimpleSerialize, Default),
        serde(deny_unknown_fields)
    )
)]
#[derive(serde::Deserialize, Debug, Clone)]
pub struct LightClientStore {
    pub finalized_header: LightClientHeader,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: Option<SyncCommittee>,
    pub optimistic_header: LightClientHeader,
    pub previous_max_active_participants: u64,
    pub current_max_active_participants: u64,
}

// impl LightClientStore {
//     pub fn init(bootstrap: Bootstrap, _trusted_block_root: Bytes32 ) -> Self {
//         // verify if the root is in sync with the bootstrap
//         LightClientStore {
//             finalized_header: bootstrap.header.clone(),
//             current_sync_committee: bootstrap.current_sync_committee,
//             next_sync_committee: None,
//             optimistic_header: bootstrap.header.clone(),
//             previous_max_active_participants: 0,
//             current_max_active_participants: 0,
//         }
//     }
// }
