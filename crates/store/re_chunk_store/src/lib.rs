//! The Rerun chunk store, implemented on top of [Apache Arrow](https://arrow.apache.org/)
//! using the [`arrow2`] crate.
//!
//! This crate is an in-memory time series database for Rerun log data.
//! It is indexed by Entity path, component, timeline, and time.
//! It supports out-of-order insertions, and fast `O(log(N))` queries.
//!
//! * See [`ChunkStore`] for an overview of the core data structures.
//! * See [`ChunkStore::latest_at_relevant_chunks`] and [`ChunkStore::range_relevant_chunks`]
//!   for the documentation of the public read APIs.
//! * See [`ChunkStore::insert_chunk`] for the documentation of the public write APIs.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!

mod dataframe;
mod events;
mod gc;
mod query;
mod shim;
mod stats;
mod store;
mod subscribers;
mod writes;

use arrow2::datatypes::DataType as ArrowDataType;
use re_types_core::{ComponentName, ComponentNameSet};
use std::collections::BTreeSet;
use std::fmt::Display;
use std::sync::Arc;

pub use self::dataframe::{
    ColumnDescriptor, ComponentColumnDescriptor, ControlColumnDescriptor, LatestAtQueryExpression,
    QueryExpression, RangeQueryExpression, TimeColumnDescriptor,
};
pub use self::events::{ChunkStoreDiff, ChunkStoreDiffKind, ChunkStoreEvent};
pub use self::gc::{GarbageCollectionOptions, GarbageCollectionTarget};
pub use self::shim::ChunkStoreShim;
pub use self::stats::{ChunkStoreChunkStats, ChunkStoreStats};
pub use self::store::{ChunkStore, ChunkStoreConfig, ChunkStoreGeneration};
pub use self::subscribers::{ChunkStoreSubscriber, ChunkStoreSubscriberHandle};

use re_chunk::EntityPath;
// Re-exports
#[doc(no_inline)]
pub use re_chunk::{
    Chunk, ChunkId, ChunkShared, LatestAtQuery, RangeQuery, RangeQueryOptions, RowId,
    UnitChunkShared,
};
#[doc(no_inline)]
pub use re_log_encoding::decoder::VersionPolicy;
use re_log_types::StoreId;
#[doc(no_inline)]
pub use re_log_types::{ResolvedTimeRange, TimeInt, TimeType, Timeline};

pub mod external {
    pub use re_chunk;
    pub use re_log_encoding;
}

// ---

#[derive(thiserror::Error, Debug)]
pub enum ChunkStoreError {
    #[error("Chunks must be sorted before insertion in the chunk store")]
    UnsortedChunk,

    #[error(transparent)]
    Chunk(#[from] re_chunk::ChunkError),

    /// Error when parsing configuration from environment.
    #[error("Failed to parse config: '{name}={value}': {err}")]
    ParseConfig {
        name: &'static str,
        value: String,
        err: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub type ChunkStoreResult<T> = ::std::result::Result<T, ChunkStoreError>;

pub trait ChunkStoreAPI: Send + Sync + Display {
    // general
    fn id(&self) -> &StoreId;
    fn generation(&self) -> ChunkStoreGeneration;
    fn config(&self) -> &ChunkStoreConfig;
    fn iter_chunks(&self) -> Box<dyn Iterator<Item = &Arc<Chunk>> + '_>;
    fn chunk(&self, id: &ChunkId) -> Option<&Arc<Chunk>>;
    fn num_chunks(&self) -> usize;
    fn lookup_datatype(&self, component_name: &ComponentName) -> Option<&ArrowDataType>;

    // dataframe
    fn schema(&self) -> Vec<ColumnDescriptor>;
    fn schema_for_query(&self, query: &QueryExpression) -> Vec<ColumnDescriptor>;

    // writers
    fn insert_chunk(&mut self, chunk: &Arc<Chunk>) -> ChunkStoreResult<Vec<ChunkStoreEvent>>;
    fn drop_entity_path(&mut self, entity_path: &EntityPath) -> Vec<ChunkStoreEvent>;

    // query
    fn all_timelines(&self) -> BTreeSet<Timeline>;
    fn all_entities(&self) -> BTreeSet<EntityPath>;
    fn all_components(&self) -> BTreeSet<ComponentName>;
    fn all_components_on_timeline(
        &self,
        timeline: &Timeline,
        entity_path: &EntityPath,
    ) -> Option<ComponentNameSet>;
    fn all_components_for_entity(&self, entity_path: &EntityPath) -> Option<ComponentNameSet>;
    fn entity_has_component_on_timeline(
        &self,
        timeline: &Timeline,
        entity_path: &EntityPath,
        component_name: &ComponentName,
    ) -> bool;
    fn entity_has_component(
        &self,
        entity_path: &EntityPath,
        component_name: &ComponentName,
    ) -> bool;
    fn entity_has_static_component(
        &self,
        entity_path: &EntityPath,
        component_name: &ComponentName,
    ) -> bool;
    fn entity_has_temporal_component(
        &self,
        entity_path: &EntityPath,
        component_name: &ComponentName,
    ) -> bool;
    fn entity_has_temporal_component_on_timeline(
        &self,
        timeline: &Timeline,
        entity_path: &EntityPath,
        component_name: &ComponentName,
    ) -> bool;
    fn entity_has_data_on_timeline(&self, timeline: &Timeline, entity_path: &EntityPath) -> bool;
    fn entity_has_data(&self, entity_path: &EntityPath) -> bool;
    fn entity_has_static_data(&self, entity_path: &EntityPath) -> bool;
    fn entity_has_temporal_data(&self, entity_path: &EntityPath) -> bool;
    fn entity_has_temporal_data_on_timeline(
        &self,
        timeline: &Timeline,
        entity_path: &EntityPath,
    ) -> bool;
    fn entity_min_time(&self, timeline: &Timeline, entity_path: &EntityPath) -> Option<TimeInt>;
    fn entity_time_range(
        &self,
        timeline: &Timeline,
        entity_path: &EntityPath,
    ) -> Option<ResolvedTimeRange>;
    fn time_range(&self, timeline: &Timeline) -> Option<ResolvedTimeRange>;
    fn latest_at_relevant_chunks(
        &self,
        query: &LatestAtQuery,
        entity_path: &EntityPath,
        component_name: ComponentName,
    ) -> Vec<Arc<Chunk>>;
    fn latest_at_relevant_chunks_for_all_components(
        &self,
        query: &LatestAtQuery,
        entity_path: &EntityPath,
    ) -> Vec<Arc<Chunk>>;
    fn range_relevant_chunks(
        &self,
        query: &RangeQuery,
        entity_path: &EntityPath,
        component_name: ComponentName,
    ) -> Vec<Arc<Chunk>>;
    fn range_relevant_chunks_for_all_components(
        &self,
        query: &RangeQuery,
        entity_path: &EntityPath,
    ) -> Vec<Arc<Chunk>>;

    // stats
    fn stats(&self) -> ChunkStoreStats;
    fn entity_stats_static(&self, entity_path: &EntityPath) -> ChunkStoreChunkStats;
    fn entity_stats_on_timeline(
        &self,
        entity_path: &EntityPath,
        timeline: &Timeline,
    ) -> ChunkStoreChunkStats;
    fn num_static_events_for_component(
        &self,
        entity_path: &EntityPath,
        component_name: ComponentName,
    ) -> u64;
    fn num_temporal_events_for_component_on_timeline(
        &self,
        timeline: &Timeline,
        entity_path: &EntityPath,
        component_name: ComponentName,
    ) -> u64;

    // gc
    fn gc(&mut self, options: &GarbageCollectionOptions)
        -> (Vec<ChunkStoreEvent>, ChunkStoreStats);
}
