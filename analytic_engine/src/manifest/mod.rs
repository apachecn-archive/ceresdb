// Copyright 2022 CeresDB Project Authors. Licensed under Apache-2.0.

//! Manage meta data of the engine

pub mod details;
pub mod meta_data;
pub mod meta_update;

use std::{fmt, sync::Arc};

use async_trait::async_trait;
use common_types::table::ShardId;
use common_util::error::GenericResult;
use table_engine::table::TableId;

use crate::{
    manifest::{meta_data::TableManifestData, meta_update::MetaUpdateRequest},
    space::SpaceId,
};

#[derive(Debug)]
pub struct LoadRequest {
    pub space_id: SpaceId,
    pub table_id: TableId,
    pub shard_id: ShardId,
}

pub type SnapshotRequest = LoadRequest;
/// Manifest holds meta data of all tables.
#[async_trait]
pub trait Manifest: Send + Sync + fmt::Debug {
    /// Store update to manifest
    async fn store_update(&self, request: MetaUpdateRequest) -> GenericResult<()>;

    /// Load table meta data from manifest.
    ///
    /// If `do_snapshot` is true, the manifest will try to create a snapshot of
    /// the manifest data.
    async fn load_data(
        &self,
        load_request: &LoadRequest,
    ) -> GenericResult<Option<TableManifestData>>;

    async fn do_snapshot(&self, request: SnapshotRequest) -> GenericResult<()>;
}

pub type ManifestRef = Arc<dyn Manifest>;
