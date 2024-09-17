use crate::{ChunkStore, ChunkStoreAPI, ChunkStoreConfig};
use re_chunk::Chunk;
use re_log_types::StoreId;
use re_types_core::{ComponentName, ComponentNameSet};

use std::{
    fmt::{self, Display},
    sync::{atomic::AtomicU64, Arc},
    thread::{self, sleep},
    time::Duration,
};

pub struct ChunkStoreShim {
    store: ChunkStore,
    stats: Arc<ShimStats>,
}

impl Display for ChunkStoreShim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChunkStoreShim")
    }
}

#[derive(Default)]
struct ShimStats {
    latest_at_count: AtomicU64,
    range_count: AtomicU64,
    id_count: AtomicU64,
    generation_count: AtomicU64,
    config_count: AtomicU64,
    iter_chunks_count: AtomicU64,
    chunk_count: AtomicU64,
    num_chunks_count: AtomicU64,
    lookup_datatype_count: AtomicU64,
    insert_chunk_count: AtomicU64,
    drop_entity_path_count: AtomicU64,
    all_timelines_count: AtomicU64,
    all_entities_count: AtomicU64,
    all_components_count: AtomicU64,
    all_component_on_timeline_count: AtomicU64,
    all_component_for_entity_count: AtomicU64,
    entity_component_on_timeline: AtomicU64,
    entity_component_count: AtomicU64,
    entity_static_component_count: AtomicU64,
    entity_temporal_component_count: AtomicU64,
    entity_temporal_component_on_timeline_count: AtomicU64,
    entity_data_on_timeline_count: AtomicU64,
    entity_data_count: AtomicU64,
    entity_static_data_count: AtomicU64,
    entitity_temporal_data_count: AtomicU64,
    entity_temporal_data_on_timeline_count: AtomicU64,
    store_min_time_count: AtomicU64,
    entity_time_range_count: AtomicU64,
    time_range_count: AtomicU64,
    entity_static_count: AtomicU64,
    entity_stats_on_timeline_count: AtomicU64,
    num_static_events_for_component_count: AtomicU64,
    num_temporal_events_for_component_on_timeline: AtomicU64,
    schema_count: AtomicU64,
    schema_for_query: AtomicU64,
    stats_count: AtomicU64,
    gc_count: AtomicU64,
}

impl Display for ShimStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "latest at count: {}, range count: {}, id_count:{}, generation_count:{}, config_count:{}, iter_chunks_count:{}, chunk_count:{}, num_chunks_count:{}, lookup_datatype_count:{},
            insert_chunk_count:{}, drop_entity_path_count:{}, all_timelines_count:{}, all_entities_count:{}, all_components_count:{}  , all_component_on_timeline_count:{}  , all_component_for_entity_count:{}  ,
            entity_component_on_timeline:{}, entity_component_count:{}  , entity_static_component_count:{}  , entity_temporal_component_count:{}  , entity_temporal_component_on_timeline_count:{}  ,
            entity_data_on_timeline_count˙{}  , entity_data_count:{}  , entity_static_data_count:{}  , entitity_temporal_data_count:{}  , entity_temporal_data_on_timeline_count:{}  , store_min_time_count:{}  ,
            entity_time_range_count:{}, time_range_count:{}  , entity_static_count:{}  , entity_stats_on_timeline_count:{}  , num_static_events_for_component_count:{}  ,
            num_temporal_events_for_component_on_timeline:{}, schema_count:{}  , schema_for_query:{}  , stats_count:{}  , gc_count:{}  ,
            ",
            self.latest_at_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.latest_at_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.id_count.load(std::sync::atomic::Ordering::Relaxed),
            self.generation_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.config_count.load(std::sync::atomic::Ordering::Relaxed),
            self.iter_chunks_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.chunk_count.load(std::sync::atomic::Ordering::Relaxed),
            self.num_chunks_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.lookup_datatype_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.insert_chunk_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.drop_entity_path_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.all_timelines_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.all_entities_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.all_components_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.all_component_on_timeline_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.all_component_for_entity_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_component_on_timeline
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_component_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_static_component_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_temporal_component_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_temporal_component_on_timeline_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_data_on_timeline_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_data_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_static_data_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entitity_temporal_data_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_temporal_data_on_timeline_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.store_min_time_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_time_range_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.time_range_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_static_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.entity_stats_on_timeline_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.num_static_events_for_component_count
                .load(std::sync::atomic::Ordering::Relaxed),
            self.num_temporal_events_for_component_on_timeline
                .load(std::sync::atomic::Ordering::Relaxed),
            self.schema_count.load(std::sync::atomic::Ordering::Relaxed),
            self.schema_for_query
                .load(std::sync::atomic::Ordering::Relaxed),
            self.stats_count.load(std::sync::atomic::Ordering::Relaxed),
            self.gc_count.load(std::sync::atomic::Ordering::Relaxed),
        )
    }
}

impl ChunkStoreShim {
    pub fn new(id: StoreId, config: ChunkStoreConfig) -> Self {
        let store = ChunkStore::new(id, config);

        let shim = Self {
            store,
            stats: Arc::new(ShimStats::default()),
        };

        // let stats = shim.stats.clone();

        // thread::spawn(move || loop {
        //     sleep(Duration::from_secs(3));
        //     println!("ChunkStoreShim is alive");
        //     print_stats(&stats);
        // });

        shim
    }
}

fn print_stats(stats: &ShimStats) {
    println!("Stats: {}", stats);
}

impl ChunkStoreAPI for ChunkStoreShim {
    fn id(&self) -> &re_log_types::StoreId {
        self.stats
            .id_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.id()
    }

    fn generation(&self) -> crate::ChunkStoreGeneration {
        self.stats
            .generation_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.generation()
    }

    fn config(&self) -> &crate::ChunkStoreConfig {
        self.stats
            .config_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.config()
    }

    fn iter_chunks(&self) -> Box<dyn Iterator<Item = &Arc<Chunk>> + '_> {
        self.stats
            .iter_chunks_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Box::new(self.store.iter_chunks())
    }

    fn chunk(&self, id: &re_chunk::ChunkId) -> Option<&std::sync::Arc<re_chunk::Chunk>> {
        self.stats
            .chunk_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.chunk(id)
    }

    fn num_chunks(&self) -> usize {
        self.stats
            .num_chunks_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.num_chunks()
    }

    fn lookup_datatype(
        &self,
        component_name: &re_chunk::ComponentName,
    ) -> Option<&arrow2::datatypes::DataType> {
        self.stats
            .lookup_datatype_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.lookup_datatype(component_name)
    }

    fn insert_chunk(
        &mut self,
        chunk: &std::sync::Arc<re_chunk::Chunk>,
    ) -> crate::ChunkStoreResult<Vec<crate::ChunkStoreEvent>> {
        self.stats
            .insert_chunk_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.insert_chunk(chunk)
    }

    fn drop_entity_path(
        &mut self,
        entity_path: &re_chunk::EntityPath,
    ) -> Vec<crate::ChunkStoreEvent> {
        self.stats
            .drop_entity_path_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.drop_entity_path(entity_path)
    }

    fn all_timelines(&self) -> std::collections::BTreeSet<re_chunk::Timeline> {
        self.stats
            .all_timelines_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.all_timelines()
    }

    fn all_entities(&self) -> std::collections::BTreeSet<re_chunk::EntityPath> {
        self.stats
            .all_entities_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.all_entities()
    }

    fn all_components(&self) -> std::collections::BTreeSet<ComponentName> {
        self.stats
            .all_components_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.all_components()
    }

    fn all_components_on_timeline(
        &self,
        timeline: &re_chunk::Timeline,
        entity_path: &re_chunk::EntityPath,
    ) -> Option<ComponentNameSet> {
        self.stats
            .all_component_on_timeline_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.all_components_on_timeline(timeline, entity_path)
    }

    fn all_components_for_entity(
        &self,
        entity_path: &re_chunk::EntityPath,
    ) -> Option<ComponentNameSet> {
        self.stats
            .all_component_for_entity_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.all_components_for_entity(entity_path)
    }

    fn entity_has_component_on_timeline(
        &self,
        timeline: &re_chunk::Timeline,
        entity_path: &re_chunk::EntityPath,
        component_name: &ComponentName,
    ) -> bool {
        self.stats
            .entity_component_on_timeline
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store
            .entity_has_component_on_timeline(timeline, entity_path, component_name)
    }

    fn entity_has_component(
        &self,
        entity_path: &re_chunk::EntityPath,
        component_name: &ComponentName,
    ) -> bool {
        self.stats
            .entity_component_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.entity_has_component(entity_path, component_name)
    }

    fn entity_has_static_component(
        &self,
        entity_path: &re_chunk::EntityPath,
        component_name: &ComponentName,
    ) -> bool {
        self.stats
            .entity_static_component_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store
            .entity_has_static_component(entity_path, component_name)
    }

    fn entity_has_temporal_component(
        &self,
        entity_path: &re_chunk::EntityPath,
        component_name: &ComponentName,
    ) -> bool {
        self.stats
            .entity_temporal_component_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store
            .entity_has_temporal_component(entity_path, component_name)
    }

    fn entity_has_temporal_component_on_timeline(
        &self,
        timeline: &re_chunk::Timeline,
        entity_path: &re_chunk::EntityPath,
        component_name: &ComponentName,
    ) -> bool {
        self.stats
            .entity_temporal_component_on_timeline_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store
            .entity_has_temporal_component_on_timeline(timeline, entity_path, component_name)
    }

    fn entity_has_data_on_timeline(
        &self,
        timeline: &re_chunk::Timeline,
        entity_path: &re_chunk::EntityPath,
    ) -> bool {
        self.stats
            .entity_data_on_timeline_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store
            .entity_has_data_on_timeline(timeline, entity_path)
    }

    fn entity_has_data(&self, entity_path: &re_chunk::EntityPath) -> bool {
        self.stats
            .entity_data_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.entity_has_data(entity_path)
    }

    fn entity_has_static_data(&self, entity_path: &re_chunk::EntityPath) -> bool {
        self.stats
            .entity_static_data_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.entity_has_static_data(entity_path)
    }

    fn entity_has_temporal_data(&self, entity_path: &re_chunk::EntityPath) -> bool {
        self.stats
            .entitity_temporal_data_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.entity_has_temporal_data(entity_path)
    }

    fn entity_has_temporal_data_on_timeline(
        &self,
        timeline: &re_chunk::Timeline,
        entity_path: &re_chunk::EntityPath,
    ) -> bool {
        self.stats
            .entity_temporal_data_on_timeline_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store
            .entity_has_temporal_data_on_timeline(timeline, entity_path)
    }

    fn entity_min_time(
        &self,
        timeline: &re_chunk::Timeline,
        entity_path: &re_chunk::EntityPath,
    ) -> Option<re_chunk::TimeInt> {
        self.stats
            .store_min_time_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.entity_min_time(timeline, entity_path)
    }

    fn entity_time_range(
        &self,
        timeline: &re_chunk::Timeline,
        entity_path: &re_chunk::EntityPath,
    ) -> Option<re_log_types::ResolvedTimeRange> {
        self.stats
            .entity_time_range_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.entity_time_range(timeline, entity_path)
    }

    fn time_range(&self, timeline: &re_chunk::Timeline) -> Option<re_log_types::ResolvedTimeRange> {
        self.stats
            .time_range_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.store.time_range(timeline)
    }

    fn latest_at_relevant_chunks(
        &self,
        query: &re_chunk::LatestAtQuery,
        entity_path: &re_chunk::EntityPath,
        component_name: re_chunk::ComponentName,
    ) -> Vec<std::sync::Arc<re_chunk::Chunk>> {
        self.stats
            .latest_at_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.store
            .latest_at_relevant_chunks(query, entity_path, component_name)
    }

    fn latest_at_relevant_chunks_for_all_components(
        &self,
        query: &re_chunk::LatestAtQuery,
        entity_path: &re_chunk::EntityPath,
    ) -> Vec<std::sync::Arc<re_chunk::Chunk>> {
        self.stats
            .latest_at_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.store
            .latest_at_relevant_chunks_for_all_components(query, entity_path)
    }

    fn range_relevant_chunks(
        &self,
        query: &re_chunk::RangeQuery,
        entity_path: &re_chunk::EntityPath,
        component_name: re_chunk::ComponentName,
    ) -> Vec<std::sync::Arc<re_chunk::Chunk>> {
        self.stats
            .range_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.store
            .range_relevant_chunks(query, entity_path, component_name)
    }

    fn range_relevant_chunks_for_all_components(
        &self,
        query: &re_chunk::RangeQuery,
        entity_path: &re_chunk::EntityPath,
    ) -> Vec<std::sync::Arc<re_chunk::Chunk>> {
        self.stats
            .range_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.store
            .range_relevant_chunks_for_all_components(query, entity_path)
    }

    fn entity_stats_static(
        &self,
        entity_path: &re_chunk::EntityPath,
    ) -> crate::ChunkStoreChunkStats {
        self.stats
            .entity_static_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.entity_stats_static(entity_path)
    }

    fn entity_stats_on_timeline(
        &self,
        entity_path: &re_chunk::EntityPath,
        timeline: &re_chunk::Timeline,
    ) -> crate::ChunkStoreChunkStats {
        self.stats
            .entity_stats_on_timeline_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.entity_stats_on_timeline(entity_path, timeline)
    }

    fn num_static_events_for_component(
        &self,
        entity_path: &re_chunk::EntityPath,
        component_name: re_chunk::ComponentName,
    ) -> u64 {
        self.stats
            .num_static_events_for_component_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store
            .num_static_events_for_component(entity_path, component_name)
    }

    fn num_temporal_events_for_component_on_timeline(
        &self,
        timeline: &re_chunk::Timeline,
        entity_path: &re_chunk::EntityPath,
        component_name: re_chunk::ComponentName,
    ) -> u64 {
        self.stats
            .num_temporal_events_for_component_on_timeline
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.num_temporal_events_for_component_on_timeline(
            timeline,
            entity_path,
            component_name,
        )
    }

    fn schema(&self) -> Vec<crate::ColumnDescriptor> {
        self.stats
            .schema_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.schema()
    }

    fn schema_for_query(&self, query: &crate::QueryExpression) -> Vec<crate::ColumnDescriptor> {
        self.stats
            .schema_for_query
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.schema_for_query(query)
    }

    fn stats(&self) -> crate::ChunkStoreStats {
        self.stats
            .stats_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.store.stats()
    }

    fn gc(
        &mut self,
        options: &crate::GarbageCollectionOptions,
    ) -> (Vec<crate::ChunkStoreEvent>, crate::ChunkStoreStats) {
        self.stats
            .gc_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.store.gc(options)
    }
}
