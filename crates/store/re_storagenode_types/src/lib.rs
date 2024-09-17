pub mod storage_api {
    tonic::include_proto!("storage_api");
}

mod remote;

pub mod external {
    pub use re_chunk_store;
    pub use re_log_types;
}

pub mod storage {
    use std::sync::Arc;

    use re_chunk_store::{
        external::{
            re_chunk::{
                self,
                external::re_log_types::{LogMsg, StoreId, StoreKind},
                ChunkError,
            },
            re_log_encoding::{self, decoder::DecodeError},
        },
        Chunk, ChunkStoreDiff, ChunkStoreDiffKind, ChunkStoreEvent, ChunkStoreGeneration,
    };

    use re_log_types::{EntityPath as LocalEntityPath, ResolvedTimeRange, TimeType};
    pub use tonic::transport::*;

    pub use crate::remote::ChunkStoreRemote;

    pub use super::storage_api::*;

    #[derive(thiserror::Error, Debug)]
    pub enum DataDecodeError {
        #[error("Error converting to protobuf enum, {err}")]
        UnexpectedEnumeration { err: prost::UnknownEnumValue },
        #[error("Error converting chunk bytes into chunk. {err}")]
        ChunkDecoding { err: ChunkError },
        #[error("...")]
        ArrowMsgDecoding { err: DecodeError },
    }

    impl TryInto<StoreId> for DataStoreId {
        type Error = DataDecodeError;

        fn try_into(self) -> Result<StoreId, Self::Error> {
            Ok(StoreId::from_string(
                match DataStoreKind::try_from(self.kind).unwrap() {
                    DataStoreKind::Blueprint => StoreKind::Blueprint,
                    DataStoreKind::Recording => StoreKind::Recording,
                },
                self.id,
            ))
        }
    }

    impl TryFrom<StoreId> for DataStoreId {
        type Error = DataDecodeError;

        fn try_from(value: StoreId) -> Result<Self, Self::Error> {
            Ok(Self {
                id: (*value.id).clone(),
                kind: match value.kind {
                    StoreKind::Blueprint => DataStoreKind::Blueprint.try_into().unwrap(),
                    StoreKind::Recording => DataStoreKind::Recording.try_into().unwrap(),
                },
            })
        }
    }

    impl TryFrom<re_chunk::LatestAtQuery> for LatestAtQuery {
        type Error = DataDecodeError;

        fn try_from(value: re_chunk::LatestAtQuery) -> Result<Self, Self::Error> {
            Ok(Self {
                timeline: Some(value.timeline().try_into().unwrap()),
                at: value.at().as_i64(),
            })
        }
    }

    impl TryInto<re_chunk::LatestAtQuery> for LatestAtQuery {
        type Error = DataDecodeError;

        fn try_into(self) -> Result<re_chunk::LatestAtQuery, Self::Error> {
            Ok(re_chunk::LatestAtQuery::new(
                self.timeline.unwrap().try_into().unwrap(),
                self.at,
            ))
        }
    }

    impl TryFrom<re_chunk::Timeline> for Timeline {
        type Error = DataDecodeError;

        fn try_from(value: re_chunk::Timeline) -> Result<Self, Self::Error> {
            Ok(Timeline {
                name: value.name().to_string(),
                typ: match value.typ() {
                    TimeType::Sequence => TimelineType::Sequence.try_into().unwrap(),
                    TimeType::Time => TimelineType::Time.try_into().unwrap(),
                },
            })
        }
    }

    impl TryInto<re_chunk::Timeline> for Timeline {
        type Error = DataDecodeError;

        fn try_into(self) -> Result<re_chunk::Timeline, Self::Error> {
            Ok(re_chunk::Timeline::new(
                self.name,
                match TimelineType::try_from(self.typ).unwrap() {
                    TimelineType::Sequence => TimeType::Sequence,
                    TimelineType::Time => TimeType::Time,
                },
            ))
        }
    }

    impl TryInto<ChunkStoreGeneration> for DataStoreGeneration {
        type Error = DataDecodeError;

        fn try_into(self) -> Result<ChunkStoreGeneration, Self::Error> {
            Ok(ChunkStoreGeneration {
                gc_id: self.gc_id,
                insert_id: self.insert_id,
            })
        }
    }

    impl TryFrom<LocalEntityPath> for EntityPath {
        type Error = DataDecodeError;

        fn try_from(value: LocalEntityPath) -> Result<Self, Self::Error> {
            Ok(EntityPath {
                hash: value.hash64(),
                part: value
                    .iter()
                    .map(|p| p.interned().as_str().into())
                    .collect::<Vec<_>>(),
            })
        }
    }

    impl TryInto<LocalEntityPath> for EntityPath {
        type Error = DataDecodeError;

        fn try_into(self) -> Result<LocalEntityPath, Self::Error> {
            Ok(LocalEntityPath::new(
                self.part.into_iter().map(|p| p.into()).collect(),
            ))
        }
    }

    impl TryFrom<ResolvedTimeRange> for TimeRange {
        type Error = DataDecodeError;

        fn try_from(value: ResolvedTimeRange) -> Result<Self, Self::Error> {
            Ok(TimeRange {
                start: value.min().as_i64(),
                end: value.max().as_i64(),
            })
        }
    }

    impl TryInto<ResolvedTimeRange> for TimeRange {
        type Error = DataDecodeError;

        fn try_into(self) -> Result<ResolvedTimeRange, Self::Error> {
            Ok(
                ResolvedTimeRange::new(self.start, self.end)
            )
        }
    }

    impl TryFrom<re_chunk::RangeQuery> for RangeQuery {
        type Error = DataDecodeError;

        fn try_from(value: re_chunk::RangeQuery) -> Result<Self, Self::Error> {
            Ok(Self {
                timeline: Some(value.timeline().try_into().unwrap()),
                range: Some(value.range().try_into().unwrap()),
                options: Some(RangeQueryOptions {
                    keep_extra_components: value.options.keep_extra_components,
                    keep_extra_timelines: value.options.keep_extra_timelines,
                    include_extended_bounds: value.options.include_extended_bounds,
                }),
            })
        }
    }

    impl TryInto<re_chunk::RangeQuery> for RangeQuery {
        type Error = DataDecodeError;

        fn try_into(self) -> Result<re_chunk::RangeQuery, Self::Error> {
            Ok(re_chunk::RangeQuery::new(
                self.timeline.unwrap().try_into().unwrap(),
                self.range.unwrap().try_into().unwrap(),
            ))
        }
    }

    // Not sure we'll really need these conversions for the PoC since the viewer would not be receiving
    // the messages anymore hence adding chunk in EntityDb shouldn't be happening. In a real world scenario
    // we'd also probably want to make our datastore requests and replies directly serializable to protobuf.
    impl TryFrom<ChunkStoreEvent> for DataStoreEvent {
        type Error = DataDecodeError;

        fn try_from(value: ChunkStoreEvent) -> Result<Self, Self::Error> {
            let log_msg = value.diff.chunk.to_arrow_msg().unwrap();
            let encoding_options = re_log_encoding::EncodingOptions::UNCOMPRESSED;
            let encoded_msg = re_log_encoding::encoder::encode_to_bytes(
                re_build_info::CrateVersion::LOCAL,
                encoding_options,
                vec![&LogMsg::ArrowMsg(value.store_id.clone(), log_msg)],
            )
            .unwrap();

            let event = Self {
                store_id: Some(DataStoreId {
                    id: (*value.store_id.id).clone(),
                    kind: match value.store_id.kind {
                        StoreKind::Blueprint => DataStoreKind::Blueprint.try_into().unwrap(),
                        StoreKind::Recording => DataStoreKind::Recording.try_into().unwrap(),
                    },
                }),
                store_generation: Some(DataStoreGeneration {
                    gc_id: value.store_generation.gc_id,
                    insert_id: value.store_generation.insert_id,
                }),
                event_id: value.event_id,
                diff: Some(DataStoreDiff {
                    kind: match value.diff.kind {
                        ChunkStoreDiffKind::Addition => {
                            DataStoreDiffKind::Addition.try_into().unwrap()
                        }
                        ChunkStoreDiffKind::Deletion => {
                            DataStoreDiffKind::Deletion.try_into().unwrap()
                        }
                    },
                    chunk_data: encoded_msg,
                }),
            };

            Ok(event)
        }
    }

    impl TryInto<ChunkStoreEvent> for DataStoreEvent {
        type Error = DataDecodeError;

        fn try_into(self) -> Result<ChunkStoreEvent, Self::Error> {
            // all nested fields are Option, see https://github.com/tokio-rs/prost/issues/223
            let store_id = self.store_id.unwrap();
            let store_kind = store_id.kind;
            // hm, this enum stuff ended up being clunky...
            let store_kind = DataStoreKind::try_from(store_kind)
                .map_err(|err| DataDecodeError::UnexpectedEnumeration { err })?;

            let store_id_kind = match store_kind {
                DataStoreKind::Blueprint => StoreKind::Blueprint,
                DataStoreKind::Recording => StoreKind::Recording,
            };

            let diff = self.diff.unwrap();
            let diff_kind = DataStoreDiffKind::try_from(diff.kind)
                .map_err(|err| DataDecodeError::UnexpectedEnumeration { err })?;
            let chunkstore_diff_kind = match diff_kind {
                DataStoreDiffKind::Addition => ChunkStoreDiffKind::Addition,
                DataStoreDiffKind::Deletion => ChunkStoreDiffKind::Deletion,
            };

            let version_policy = re_log_encoding::decoder::VersionPolicy::Warn;
            let log_msgs = re_log_encoding::decoder::decode_bytes(version_policy, &diff.chunk_data)
                .map_err(|err| DataDecodeError::ArrowMsgDecoding { err })?;
            let log_msg = log_msgs.first().unwrap();
            let chunk = match log_msg {
                LogMsg::ArrowMsg(_, arrow_msg) => Chunk::from_arrow_msg(arrow_msg)
                    .map_err(|err| DataDecodeError::ChunkDecoding { err })?,
                _ => todo!(),
            };

            let diff = ChunkStoreDiff {
                kind: chunkstore_diff_kind,
                chunk: Arc::new(chunk),
                compacted: None,
            };

            Ok(ChunkStoreEvent {
                store_id: StoreId::from_string(store_id_kind, store_id.id),
                store_generation: ChunkStoreGeneration {
                    gc_id: self.store_generation.unwrap().gc_id,
                    insert_id: self.store_generation.unwrap().insert_id,
                },
                event_id: self.event_id,
                diff,
            })
        }
    }
}
