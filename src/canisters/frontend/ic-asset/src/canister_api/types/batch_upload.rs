use crate::asset::config::HeadersConfig;
use candid::{CandidType, Nat};
use serde::Deserialize;

/// Create a new batch, which will expire after some time period.
/// This expiry is extended by any call to create_chunk().
/// Also, removes any expired batches.
#[derive(CandidType, Debug)]
pub struct CreateBatchRequest {}

/// The response to a CreateBatchRequest.
#[derive(CandidType, Debug, Deserialize)]
pub struct CreateBatchResponse {
    /// The ID of the created batch.
    pub batch_id: Nat,
}

/// Upload a chunk of data that is part of an asset's content.
#[derive(CandidType, Debug, Deserialize)]
pub struct CreateChunkRequest<'a> {
    /// The batch with which to associate the created chunk.
    /// The chunk will be deleted if the batch expires before being committed.
    pub batch_id: Nat,

    /// The data in this chunk.
    #[serde(with = "serde_bytes")]
    pub content: &'a [u8],
}

/// The responst to a CreateChunkRequest.
#[derive(CandidType, Debug, Deserialize)]
pub struct CreateChunkResponse {
    /// The ID of the created chunk.
    pub chunk_id: Nat,
}

/// Create a new asset.  Has no effect if the asset already exists and the content type matches.
/// Traps if the asset already exists but with a different content type.
#[derive(CandidType, Debug)]
pub struct CreateAssetArguments {
    /// The key identifies the asset.
    pub key: String,
    /// The MIME type of this asset
    pub content_type: String,
    /// The cache HTTP header Time To Live parameter
    pub max_age: Option<u64>,
    /// The HTTP headers
    pub headers: Option<HeadersConfig>,
    /// Aliasing enabled or not
    pub enable_aliasing: Option<bool>,
    /// When set to true, don't redirect from raw to certified
    pub allow_raw_access: Option<bool>,
}

/// Set the data for a particular content encoding for the given asset.
#[derive(CandidType, Debug)]
pub struct SetAssetContentArguments {
    /// The key identifies the asset.
    pub key: String,
    /// The content encoding for which this content applies
    pub content_encoding: String,
    /// The chunks to assign to this content
    pub chunk_ids: Vec<Nat>,
    /// The sha256 of the entire content
    pub sha256: Option<Vec<u8>>,
}

/// Remove a specific content encoding for the asset.
#[derive(CandidType, Debug)]
pub struct UnsetAssetContentArguments {
    /// The key identifies the asset.
    pub key: String,
    /// The content encoding to remove.
    pub content_encoding: String,
}

/// Remove the specified asset.
#[derive(CandidType, Debug)]
pub struct DeleteAssetArguments {
    /// The key identifies the asset to delete.
    pub key: String,
}

/// Remove all assets, batches, and chunks, and reset the next batch and chunk IDs.
#[derive(CandidType, Debug)]
pub struct ClearArguments {}

/// Batch operations that can be applied atomically.
#[derive(CandidType, Debug)]
#[allow(dead_code)]
pub enum BatchOperationKind {
    /// Create a new asset.
    CreateAsset(CreateAssetArguments),

    /// Assign content to an asset by encoding.
    SetAssetContent(SetAssetContentArguments),

    /// Remove content from an asset by encoding.
    UnsetAssetContent(UnsetAssetContentArguments),

    /// Remove an asset altogether.
    DeleteAsset(DeleteAssetArguments),

    /// Clear all state from the asset canister.
    Clear(ClearArguments),
}

/// Apply all of the operations in the batch, and then remove the batch.
#[derive(CandidType, Debug)]
pub struct CommitBatchArguments {
    /// The batch to commit.
    pub batch_id: Nat,

    /// The operations to apply atomically.
    pub operations: Vec<BatchOperationKind>,
}