// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AffixFuzzer22 {
    pub fixed_sized_native: [u8; 4usize],
}

::re_types_core::macros::impl_into_cow!(AffixFuzzer22);

impl ::re_types_core::Loggable for AffixFuzzer22 {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::Struct(Fields::from(vec![Field::new(
            "fixed_sized_native",
            DataType::FixedSizeList(
                std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
                4,
            ),
            false,
        )]))
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
            let fields = Fields::from(vec![Field::new(
                "fixed_sized_native",
                DataType::FixedSizeList(
                    std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
                    4,
                ),
                false,
            )]);
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            as_array_ref(StructArray::new(
                fields,
                vec![{
                    let (somes, fixed_sized_native): (Vec<_>, Vec<_>) = data
                        .iter()
                        .map(|datum| {
                            let datum =
                                datum.as_ref().map(|datum| datum.fixed_sized_native.clone());
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let fixed_sized_native_validity: Option<arrow::buffer::NullBuffer> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    {
                        let fixed_sized_native_inner_data: Vec<_> = fixed_sized_native
                            .into_iter()
                            .flat_map(|v| match v {
                                Some(v) => itertools::Either::Left(v.into_iter()),
                                None => itertools::Either::Right(
                                    std::iter::repeat(Default::default()).take(4usize),
                                ),
                            })
                            .collect();
                        let fixed_sized_native_inner_validity: Option<arrow::buffer::NullBuffer> =
                            fixed_sized_native_validity.as_ref().map(|validity| {
                                validity
                                    .iter()
                                    .map(|b| std::iter::repeat(b).take(4usize))
                                    .flatten()
                                    .collect::<Vec<_>>()
                                    .into()
                            });
                        as_array_ref(FixedSizeListArray::new(
                            std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
                            4,
                            as_array_ref(PrimitiveArray::<UInt8Type>::new(
                                ScalarBuffer::from(
                                    fixed_sized_native_inner_data
                                        .into_iter()
                                        .collect::<Vec<_>>(),
                                ),
                                fixed_sized_native_inner_validity,
                            )),
                            fixed_sized_native_validity,
                        ))
                    }
                }],
                validity,
            ))
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
                .downcast_ref::<arrow::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.testing.datatypes.AffixFuzzer22")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.columns());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name().as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let fixed_sized_native = {
                    if !arrays_by_name.contains_key("fixed_sized_native") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "fixed_sized_native",
                        ))
                        .with_context("rerun.testing.datatypes.AffixFuzzer22");
                    }
                    let arrow_data = &**arrays_by_name["fixed_sized_native"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow::array::FixedSizeListArray>()
                            .ok_or_else(|| {
                                let expected = DataType::FixedSizeList(
                                    std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
                                    4,
                                );
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context(
                                "rerun.testing.datatypes.AffixFuzzer22#fixed_sized_native",
                            )?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let offsets = (0..)
                                .step_by(4usize)
                                .zip((4usize..).step_by(4usize).take(arrow_data.len()));
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
                                    .with_context(
                                        "rerun.testing.datatypes.AffixFuzzer22#fixed_sized_native",
                                    )?
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            };
                            ZipValidity::new_with_validity(offsets, arrow_data.nulls())
                                .map(|elem| {
                                    elem.map(|(start, end): (usize, usize)| {
                                        debug_assert!(end - start == 4usize);
                                        if arrow_data_inner.len() < end {
                                            return Err(DeserializationError::offset_slice_oob(
                                                (start, end),
                                                arrow_data_inner.len(),
                                            ));
                                        }

                                        #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                        let data =
                                            unsafe { arrow_data_inner.get_unchecked(start..end) };
                                        let data =
                                            data.iter().cloned().map(Option::unwrap_or_default);

                                        // NOTE: Unwrapping cannot fail: the length must be correct.
                                        #[allow(clippy::unwrap_used)]
                                        Ok(array_init::from_iter(data).unwrap())
                                    })
                                    .transpose()
                                })
                                .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                ZipValidity::new_with_validity(
                    ::itertools::izip!(fixed_sized_native),
                    arrow_data.nulls(),
                )
                .map(|opt| {
                    opt.map(|(fixed_sized_native)| {
                        Ok(Self {
                            fixed_sized_native: fixed_sized_native
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.testing.datatypes.AffixFuzzer22#fixed_sized_native",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.datatypes.AffixFuzzer22")?
            }
        })
    }
}

impl From<[u8; 4usize]> for AffixFuzzer22 {
    #[inline]
    fn from(fixed_sized_native: [u8; 4usize]) -> Self {
        Self { fixed_sized_native }
    }
}

impl From<AffixFuzzer22> for [u8; 4usize] {
    #[inline]
    fn from(value: AffixFuzzer22) -> Self {
        value.fixed_sized_native
    }
}

impl ::re_byte_size::SizeBytes for AffixFuzzer22 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.fixed_sized_native.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <[u8; 4usize]>::is_pod()
    }
}
