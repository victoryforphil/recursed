#![allow(clippy::unwrap_used)]
#![allow(clippy::todo)]

use re_chunk_store::{
    external::re_chunk::external::re_log_types::StoreId, Chunk, ChunkStoreAPI, ChunkStoreConfig,
    ChunkStoreGeneration, ChunkStoreStats,
};
use re_log_types::{external::re_types_core::ComponentName, LogMsg};
use tonic::transport::Channel;

use std::{collections::BTreeSet, fmt, sync::Arc};

use crate::storage::{
    storage_node_client::StorageNodeClient, AllComponentsForEntityRequest,
    AllComponentsOnTimelineRequest, DataStoreId, EntityHasComponentOnTimelineRequest,
    EntityHasComponentRequest, EntityHasDataOnTimelineRequest, EntityHasDataRequest, EntityPath,
    LatestAtQuery, LatestAtRelevantChunksRequest, RangeQuery,
    RangeRelevantChunksAllComponentsRequest, RangeRelevantChunksRequest, Timeline,
};

pub struct ChunkStoreRemote {
    id: re_log_types::StoreId,
    ds_client: StorageNodeClient<Channel>,
    rt: tokio::runtime::Runtime,
    config: ChunkStoreConfig,
}

impl ChunkStoreRemote {
    pub fn new(id: StoreId, config: ChunkStoreConfig) -> Self {
        // build current thread runtime that all async requests will run on
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        // note: https://github.com/hyperium/tonic/issues/942
        let ds_client = rt.block_on(async {
            StorageNodeClient::connect("http://127.0.0.1:51234")
                .await
                .expect("failed to connect to Storage node")
        });

        Self {
            id,
            ds_client,
            rt,
            config,
        }
    }
}

impl fmt::Display for ChunkStoreRemote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic here
        write!(f, "ChunkStoreRemote")
    }
}

impl ChunkStoreAPI for ChunkStoreRemote {
    fn id(&self) -> &StoreId {
        &self.id
    }

    fn generation(&self) -> re_chunk_store::ChunkStoreGeneration {
        let gen = self
            .rt
            .block_on(async {
                self.ds_client
                    .clone()
                    .generation(DataStoreId::try_from(self.id.clone()).unwrap())
                    .await
            })
            .unwrap()
            .into_inner();

        ChunkStoreGeneration {
            insert_id: gen.insert_id,
            gc_id: gen.gc_id,
        }
    }

    fn config(&self) -> &re_chunk_store::ChunkStoreConfig {
        &self.config
    }

    fn all_components_on_timeline(
        &self,
        timeline: &re_chunk_store::Timeline,
        entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> Option<re_chunk_store::external::re_chunk::external::re_log_types::external::re_types_core::ComponentNameSet>{
        let resp = self.rt.block_on(async {
            self.ds_client
                .clone()
                .all_components_on_timeline(AllComponentsOnTimelineRequest {
                    store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                    timeline: Some(Timeline::try_from(*timeline).unwrap()),
                    entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                })
                .await
                .unwrap()
                .into_inner()
        });

        if let Some(cs) = resp.components_set {
            Some(
                cs.component_names
                    .into_iter()
                    .map(ComponentName::from)
                    .collect::<BTreeSet<_>>(),
            )
        } else {
            None
        }
    }

    fn entity_has_data_on_timeline(
        &self,
        timeline: &re_chunk_store::Timeline,
        entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> bool {
        self.rt.block_on(async {
            self.ds_client
                .clone()
                .entity_has_data_on_timeline(EntityHasDataOnTimelineRequest {
                    store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                    timeline: Some(Timeline::try_from(*timeline).unwrap()),
                    entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                })
                .await
                .unwrap()
                .into_inner()
                .has_data
        })
    }

    fn all_components_for_entity(&self, entity_path: &re_chunk_store::external::re_chunk::EntityPath) -> Option<re_chunk_store::external::re_chunk::external::re_log_types::external::re_types_core::ComponentNameSet>{
        self.rt.block_on(async {
            let resp = self
                .ds_client
                .clone()
                .all_components_for_entity(AllComponentsForEntityRequest {
                    store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                    entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                })
                .await
                .unwrap()
                .into_inner();

            if let Some(cs) = resp.components_set {
                Some(
                    cs.component_names
                        .into_iter()
                        .map(ComponentName::from)
                        .collect::<BTreeSet<_>>(),
                )
            } else {
                None
            }
        })
    }

    fn entity_has_component_on_timeline(
        &self,
        timeline: &re_chunk_store::Timeline,
        entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        component_name: &re_chunk_store::external::re_chunk::ComponentName,
    ) -> bool {
        let resp = self
            .rt
            .block_on(async {
                self.ds_client
                    .clone()
                    .entity_has_component_on_timeline(EntityHasComponentOnTimelineRequest {
                        store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                        timeline: Some(Timeline::try_from(*timeline).unwrap()),
                        entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                        component_name: component_name.full_name().to_owned(),
                    })
                    .await
                    .unwrap()
            })
            .into_inner();

        resp.has_component
    }

    fn entity_has_component(
        &self,
        entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        component_name: &re_chunk_store::external::re_chunk::ComponentName,
    ) -> bool {
        self.rt.block_on(async {
            self.ds_client
                .clone()
                .entity_has_component(EntityHasComponentRequest {
                    store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                    entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                    component_name: component_name.full_name().to_owned(),
                })
                .await
                .unwrap()
                .into_inner()
                .has_component
        })
    }

    fn entity_has_data(
        &self,
        entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> bool {
        self.rt.block_on(async {
            self.ds_client
                .clone()
                .entity_has_data(EntityHasDataRequest {
                    store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                    entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                })
                .await
                .unwrap()
                .into_inner()
                .has_data
        })
    }

    fn latest_at_relevant_chunks(
        &self,
        query: &re_chunk_store::LatestAtQuery,
        entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        component_name: re_chunk_store::external::re_chunk::ComponentName,
    ) -> Vec<std::sync::Arc<Chunk>> {
        let resp = self
            .rt
            .block_on(async {
                self.ds_client
                    .clone()
                    .latest_at_relevant_chunks(LatestAtRelevantChunksRequest {
                        store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                        query: Some(LatestAtQuery::try_from(query.clone()).unwrap()),
                        entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                        component_name: component_name.full_name().to_owned(),
                    })
                    .await
                    .unwrap()
            })
            .into_inner();

        let version_policy = re_log_encoding::decoder::VersionPolicy::Warn;
        let log_msgs =
            re_log_encoding::decoder::decode_bytes(version_policy, &resp.chunk_data).unwrap();
        log_msgs
            .into_iter()
            .map(|log_msg| match log_msg {
                LogMsg::ArrowMsg(_, arrow_msg) => {
                    Arc::new(Chunk::from_arrow_msg(&arrow_msg).unwrap())
                }
                _ => unreachable!(),
            })
            .collect()
    }

    fn schema_for_query(
        &self,
        _query: &re_chunk_store::QueryExpression,
    ) -> Vec<re_chunk_store::ColumnDescriptor> {
        todo!()
    }

    fn insert_chunk(
        &mut self,
        _chunk: &std::sync::Arc<re_chunk_store::Chunk>,
    ) -> re_chunk_store::ChunkStoreResult<Vec<re_chunk_store::ChunkStoreEvent>> {
        todo!()
    }

    fn drop_entity_path(
        &mut self,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> Vec<re_chunk_store::ChunkStoreEvent> {
        todo!()
    }

    fn all_timelines(&self) -> std::collections::BTreeSet<re_chunk_store::Timeline> {
        todo!()
    }

    fn all_entities(
        &self,
    ) -> std::collections::BTreeSet<re_chunk_store::external::re_chunk::EntityPath> {
        todo!()
    }

    fn all_components(
        &self,
    ) -> std::collections::BTreeSet<re_chunk_store::external::re_chunk::ComponentName> {
        todo!()
    }

    fn iter_chunks(&self) -> Box<dyn Iterator<Item = &std::sync::Arc<re_chunk_store::Chunk>> + '_> {
        todo!()
    }

    fn chunk(
        &self,
        _id: &re_chunk_store::ChunkId,
    ) -> Option<&std::sync::Arc<re_chunk_store::Chunk>> {
        todo!()
    }

    fn num_chunks(&self) -> usize {
        todo!()
    }

    fn lookup_datatype(
        &self,
        _component_name: &re_chunk_store::external::re_chunk::ComponentName,
    ) -> Option<&re_chunk_store::external::re_chunk::external::arrow2::datatypes::DataType> {
        todo!()
    }

    fn schema(&self) -> Vec<re_chunk_store::ColumnDescriptor> {
        todo!()
    }

    fn entity_has_static_component(
        &self,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        _component_name: &re_chunk_store::external::re_chunk::ComponentName,
    ) -> bool {
        todo!()
    }

    fn entity_has_temporal_component(
        &self,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        _component_name: &re_chunk_store::external::re_chunk::ComponentName,
    ) -> bool {
        todo!()
    }

    fn entity_has_temporal_component_on_timeline(
        &self,
        _timeline: &re_chunk_store::Timeline,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        _component_name: &re_chunk_store::external::re_chunk::ComponentName,
    ) -> bool {
        todo!()
    }

    fn entity_has_static_data(
        &self,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> bool {
        todo!()
    }

    fn entity_has_temporal_data(
        &self,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> bool {
        todo!()
    }

    fn entity_has_temporal_data_on_timeline(
        &self,
        _timeline: &re_chunk_store::Timeline,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> bool {
        todo!()
    }

    fn entity_min_time(
        &self,
        _timeline: &re_chunk_store::Timeline,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> Option<re_chunk_store::TimeInt> {
        todo!()
    }

    fn entity_time_range(
        &self,
        _timeline: &re_chunk_store::Timeline,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> Option<re_chunk_store::ResolvedTimeRange> {
        todo!()
    }

    fn time_range(
        &self,
        _timeline: &re_chunk_store::Timeline,
    ) -> Option<re_chunk_store::ResolvedTimeRange> {
        todo!()
    }

    fn range_relevant_chunks_for_all_components(
        &self,
        query: &re_chunk_store::RangeQuery,
        entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> Vec<std::sync::Arc<re_chunk_store::Chunk>> {
        self.rt.block_on(async {
            let resp = self
                .ds_client
                .clone()
                .range_relevant_chunks_all_components(RangeRelevantChunksAllComponentsRequest {
                    store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                    query: Some(RangeQuery::try_from(query.clone()).unwrap()),
                    entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                })
                .await
                .unwrap()
                .into_inner();

            let version_policy = re_log_encoding::decoder::VersionPolicy::Warn;
            let log_msgs =
                re_log_encoding::decoder::decode_bytes(version_policy, &resp.chunk_data).unwrap();
            log_msgs
                .into_iter()
                .map(|log_msg| match log_msg {
                    LogMsg::ArrowMsg(_, arrow_msg) => {
                        Arc::new(Chunk::from_arrow_msg(&arrow_msg).unwrap())
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
    }

    fn latest_at_relevant_chunks_for_all_components(
        &self,
        _query: &re_chunk_store::LatestAtQuery,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> Vec<std::sync::Arc<re_chunk_store::Chunk>> {
        todo!()
    }

    fn range_relevant_chunks(
        &self,
        query: &re_chunk_store::RangeQuery,
        entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        component_name: re_chunk_store::external::re_chunk::ComponentName,
    ) -> Vec<std::sync::Arc<re_chunk_store::Chunk>> {
        self.rt.block_on(async {
            let resp = self
                .ds_client
                .clone()
                .range_relevant_chunks(RangeRelevantChunksRequest {
                    store_id: Some(DataStoreId::try_from(self.id.clone()).unwrap()),
                    query: Some(RangeQuery::try_from(query.clone()).unwrap()),
                    entity_path: Some(EntityPath::try_from(entity_path.clone()).unwrap()),
                    component_name: component_name.full_name().to_owned(),
                })
                .await
                .unwrap()
                .into_inner();

            let version_policy = re_log_encoding::decoder::VersionPolicy::Warn;
            let log_msgs =
                re_log_encoding::decoder::decode_bytes(version_policy, &resp.chunk_data).unwrap();
            log_msgs
                .into_iter()
                .map(|log_msg| match log_msg {
                    LogMsg::ArrowMsg(_, arrow_msg) => {
                        Arc::new(Chunk::from_arrow_msg(&arrow_msg).unwrap())
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
    }

    fn stats(&self) -> re_chunk_store::ChunkStoreStats {
        ChunkStoreStats::default()
    }

    fn entity_stats_static(
        &self,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
    ) -> re_chunk_store::ChunkStoreChunkStats {
        todo!()
    }

    fn entity_stats_on_timeline(
        &self,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        _timeline: &re_chunk_store::Timeline,
    ) -> re_chunk_store::ChunkStoreChunkStats {
        todo!()
    }

    fn num_static_events_for_component(
        &self,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        _component_name: re_chunk_store::external::re_chunk::ComponentName,
    ) -> u64 {
        todo!()
    }

    fn num_temporal_events_for_component_on_timeline(
        &self,
        _timeline: &re_chunk_store::Timeline,
        _entity_path: &re_chunk_store::external::re_chunk::EntityPath,
        _component_name: re_chunk_store::external::re_chunk::ComponentName,
    ) -> u64 {
        todo!()
    }

    fn gc(
        &mut self,
        _options: &re_chunk_store::GarbageCollectionOptions,
    ) -> (
        Vec<re_chunk_store::ChunkStoreEvent>,
        re_chunk_store::ChunkStoreStats,
    ) {
        re_log::info!("ignoring a gc call");

        (vec![], re_chunk_store::ChunkStoreStats::default())
    }
}
