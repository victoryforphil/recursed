// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/blob.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: A binary blob of data.
///
/// Ref-counted internally and therefore cheap to clone.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Blob(pub ::re_types_core::ArrowBuffer<u8>);

::re_types_core::macros::impl_into_cow!(Blob);

impl ::re_types_core::Loggable for Blob {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::List(std::sync::Arc::new(Field::new(
            "item",
            DataType::UInt8,
            false,
        )))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{arrow_helpers::as_array_ref, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().0);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                let offsets = arrow::buffer::OffsetBuffer::<i32>::from_lengths(
                    data0
                        .iter()
                        .map(|opt| opt.as_ref().map_or(0, |datum| datum.num_instances())),
                );
                let data0_inner_data: ScalarBuffer<_> = data0
                    .iter()
                    .flatten()
                    .map(|b| b.as_slice())
                    .collect::<Vec<_>>()
                    .concat()
                    .into();
                let data0_inner_validity: Option<arrow::buffer::NullBuffer> = None;
                as_array_ref(ListArray::try_new(
                    std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
                    offsets,
                    as_array_ref(PrimitiveArray::<UInt8Type>::new(
                        data0_inner_data,
                        data0_inner_validity,
                    )),
                    data0_validity,
                )?)
            }
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow::array::ListArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.Blob#data")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let arrow_data_inner = {
                    let arrow_data_inner = &**arrow_data.values();
                    arrow_data_inner
                        .as_any()
                        .downcast_ref::<UInt8Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt8;
                            let actual = arrow_data_inner.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.Blob#data")?
                        .values()
                };
                let offsets = arrow_data.offsets();
                ZipValidity::new_with_validity(offsets.windows(2), arrow_data.nulls())
                    .map(|elem| {
                        elem.map(|window| {
                            let start = window[0] as usize;
                            let end = window[1] as usize;
                            if arrow_data_inner.len() < end {
                                return Err(DeserializationError::offset_slice_oob(
                                    (start, end),
                                    arrow_data_inner.len(),
                                ));
                            }

                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                            let data = arrow_data_inner.clone().slice(start, end - start);
                            let data = ::re_types_core::ArrowBuffer::from(data);
                            Ok(data)
                        })
                        .transpose()
                    })
                    .collect::<DeserializationResult<Vec<Option<_>>>>()?
            }
            .into_iter()
        }
        .map(|v| v.ok_or_else(DeserializationError::missing_data))
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<DeserializationResult<Vec<Option<_>>>>()
        .with_context("rerun.datatypes.Blob#data")
        .with_context("rerun.datatypes.Blob")?)
    }
}

impl From<::re_types_core::ArrowBuffer<u8>> for Blob {
    #[inline]
    fn from(data: ::re_types_core::ArrowBuffer<u8>) -> Self {
        Self(data)
    }
}

impl From<Blob> for ::re_types_core::ArrowBuffer<u8> {
    #[inline]
    fn from(value: Blob) -> Self {
        value.0
    }
}

impl ::re_byte_size::SizeBytes for Blob {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <::re_types_core::ArrowBuffer<u8>>::is_pod()
    }
}
