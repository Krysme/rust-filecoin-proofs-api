#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![allow(clippy::too_many_arguments)]
//requires nightly, or later stable version
//#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]

pub mod post;
pub mod seal;

mod registry;
mod types;

pub use crate::registry::{RegisteredAggregationProof, RegisteredPoStProof, RegisteredSealProof};
pub use crate::types::{PrivateReplicaInfo, PublicReplicaInfo};

pub use filecoin_proofs_v1::types::{
    AggregateSnarkProof, ChallengeSeed, Commitment, PaddedBytesAmount, PieceInfo, PoStType,
    ProverId, Ticket, UnpaddedByteIndex, UnpaddedBytesAmount,
};
pub use filecoin_proofs_v1::{FallbackPoStSectorProof, SnarkProof, VanillaProof};
pub use fr32;
pub use storage_proofs_core::{
    api_version::ApiVersion,
    error::Error as StorageProofsError,
    merkle::MerkleTreeTrait,
    parameter_cache::{get_parameter_data, get_verifying_key_data},
    sector::{OrderedSectorSet, SectorId},
    util::NODE_SIZE,
};

pub const GIT_VERSION: &str = git_version::git_version!(
    args = ["--abbrev=40", "--always", "--dirty=-modified"],
    prefix = "git:"
);
pub const RUST_FIL_PROOFS_GIT_VERSION: &str = filecoin_proofs_v1::GIT_VERSION;
