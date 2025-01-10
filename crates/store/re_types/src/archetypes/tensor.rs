// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/tensor.fbs".

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

/// **Archetype**: An N-dimensional array of numbers.
///
/// ## Example
///
/// ### Simple tensor
/// ```ignore
/// use ndarray::{Array, ShapeBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_tensor").spawn()?;
///
///     let mut data = Array::<u8, _>::default((8, 6, 3, 5).f());
///     data.map_inplace(|x| *x = rand::random());
///
///     let tensor =
///         rerun::Tensor::try_from(data)?.with_dim_names(["width", "height", "channel", "batch"]);
///     rec.log("tensor", &tensor)?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/tensor_simple/baacb07712f7b706e3c80e696f70616c6c20b367/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/tensor_simple/baacb07712f7b706e3c80e696f70616c6c20b367/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/tensor_simple/baacb07712f7b706e3c80e696f70616c6c20b367/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/tensor_simple/baacb07712f7b706e3c80e696f70616c6c20b367/1200w.png">
///   <img src="https://static.rerun.io/tensor_simple/baacb07712f7b706e3c80e696f70616c6c20b367/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    /// The tensor data
    pub data: crate::components::TensorData,

    /// The expected range of values.
    ///
    /// This is typically the expected range of valid values.
    /// Everything outside of the range is clamped to the range for the purpose of colormpaping.
    /// Any colormap applied for display, will map this range.
    ///
    /// If not specified, the range will be automatically estimated from the data.
    /// Note that the Viewer may try to guess a wider range than the minimum/maximum of values
    /// in the contents of the tensor.
    /// E.g. if all values are positive, some bigger than 1.0 and all smaller than 255.0,
    /// the Viewer will guess that the data likely came from an 8bit image, thus assuming a range of 0-255.
    pub value_range: Option<crate::components::ValueRange>,
}

impl Tensor {
    /// Returns the [`ComponentDescriptor`] for [`Self::data`].
    #[inline]
    pub fn descriptor_data() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Tensor".into()),
            component_name: "rerun.components.TensorData".into(),
            archetype_field_name: Some("data".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::value_range`].
    #[inline]
    pub fn descriptor_value_range() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Tensor".into()),
            component_name: "rerun.components.ValueRange".into(),
            archetype_field_name: Some("value_range".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Tensor".into()),
            component_name: "rerun.components.TensorIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Tensor::descriptor_data()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Tensor::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Tensor::descriptor_value_range()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Tensor::descriptor_data(),
            Tensor::descriptor_indicator(),
            Tensor::descriptor_value_range(),
        ]
    });

impl Tensor {
    /// The total number of components in the archetype: 1 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`Tensor`] [`::re_types_core::Archetype`]
pub type TensorIndicator = ::re_types_core::GenericIndicatorComponent<Tensor>;

impl ::re_types_core::Archetype for Tensor {
    type Indicator = TensorIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Tensor".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Tensor"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: TensorIndicator = TensorIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let data = {
            let array = arrays_by_descr
                .get(&Self::descriptor_data())
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Tensor#data")?;
            <crate::components::TensorData>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Tensor#data")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Tensor#data")?
        };
        let value_range = if let Some(array) = arrays_by_descr.get(&Self::descriptor_value_range())
        {
            <crate::components::ValueRange>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Tensor#value_range")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { data, value_range })
    }
}

impl ::re_types_core::AsComponents for Tensor {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.data as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(Self::descriptor_data()),
                }
            }),
            (self
                .value_range
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_value_range()),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Tensor {}

impl Tensor {
    /// Create a new `Tensor`.
    #[inline]
    pub fn new(data: impl Into<crate::components::TensorData>) -> Self {
        Self {
            data: data.into(),
            value_range: None,
        }
    }

    /// The expected range of values.
    ///
    /// This is typically the expected range of valid values.
    /// Everything outside of the range is clamped to the range for the purpose of colormpaping.
    /// Any colormap applied for display, will map this range.
    ///
    /// If not specified, the range will be automatically estimated from the data.
    /// Note that the Viewer may try to guess a wider range than the minimum/maximum of values
    /// in the contents of the tensor.
    /// E.g. if all values are positive, some bigger than 1.0 and all smaller than 255.0,
    /// the Viewer will guess that the data likely came from an 8bit image, thus assuming a range of 0-255.
    #[inline]
    pub fn with_value_range(
        mut self,
        value_range: impl Into<crate::components::ValueRange>,
    ) -> Self {
        self.value_range = Some(value_range.into());
        self
    }
}

impl ::re_byte_size::SizeBytes for Tensor {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.data.heap_size_bytes() + self.value_range.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::TensorData>::is_pod()
            && <Option<crate::components::ValueRange>>::is_pod()
    }
}
