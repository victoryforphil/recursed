// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/clear.fbs".

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

use crate::try_serialize_field;
use crate::SerializationResult;
use crate::{ComponentBatch, ComponentBatchCowWithDescriptor, SerializedComponentBatch};
use crate::{ComponentDescriptor, ComponentName};
use crate::{DeserializationError, DeserializationResult};

/// **Archetype**: Empties all the components of an entity.
///
/// The presence of a clear means that a latest-at query of components at a given path(s)
/// will not return any components that were logged at those paths before the clear.
/// Any logged components after the clear are unaffected by the clear.
///
/// This implies that a range query that includes time points that are before the clear,
/// still returns all components at the given path(s).
/// Meaning that in practice clears are ineffective when making use of visible time ranges.
/// Scalar plots are an exception: they track clears and use them to represent holes in the
/// data (i.e. discontinuous lines).
///
/// ## Example
///
/// ### Flat
/// ```ignore
/// use rerun::external::glam;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_clear").spawn()?;
///
///     #[rustfmt::skip]
///     let (vectors, origins, colors) = (
///         [glam::Vec3::X,    glam::Vec3::NEG_Y, glam::Vec3::NEG_X, glam::Vec3::Y],
///         [(-0.5, 0.5, 0.0), (0.5, 0.5, 0.0),   (0.5, -0.5, 0.0),  (-0.5, -0.5, 0.0)],
///         [(200, 0, 0),      (0, 200, 0),       (0, 0, 200),       (200, 0, 200)],
///     );
///
///     // Log a handful of arrows.
///     for (i, ((vector, origin), color)) in vectors.into_iter().zip(origins).zip(colors).enumerate() {
///         rec.log(
///             format!("arrows/{i}"),
///             &rerun::Arrows3D::from_vectors([vector])
///                 .with_origins([origin])
///                 .with_colors([rerun::Color::from_rgb(color.0, color.1, color.2)]),
///         )?;
///     }
///
///     // Now clear them, one by one on each tick.
///     for i in 0..vectors.len() {
///         rec.log(format!("arrows/{i}"), &rerun::Clear::flat())?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/clear_simple/2f5df95fcc53e9f0552f65670aef7f94830c5c1a/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/clear_simple/2f5df95fcc53e9f0552f65670aef7f94830c5c1a/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/clear_simple/2f5df95fcc53e9f0552f65670aef7f94830c5c1a/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/clear_simple/2f5df95fcc53e9f0552f65670aef7f94830c5c1a/1200w.png">
///   <img src="https://static.rerun.io/clear_simple/2f5df95fcc53e9f0552f65670aef7f94830c5c1a/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clear {
    pub is_recursive: crate::components::ClearIsRecursive,
}

impl Clear {
    /// Returns the [`ComponentDescriptor`] for [`Self::is_recursive`].
    #[inline]
    pub fn descriptor_is_recursive() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Clear".into()),
            component_name: "rerun.components.ClearIsRecursive".into(),
            archetype_field_name: Some("is_recursive".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Clear".into()),
            component_name: "rerun.components.ClearIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Clear::descriptor_is_recursive()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Clear::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Clear::descriptor_is_recursive(),
            Clear::descriptor_indicator(),
        ]
    });

impl Clear {
    /// The total number of components in the archetype: 1 required, 1 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`Clear`] [`crate::Archetype`]
pub type ClearIndicator = crate::GenericIndicatorComponent<Clear>;

impl crate::Archetype for Clear {
    type Indicator = ClearIndicator;

    #[inline]
    fn name() -> crate::ArchetypeName {
        "rerun.archetypes.Clear".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Clear"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: ClearIndicator = ClearIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn crate::ComponentBatch)
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
        use crate::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let is_recursive = {
            let array = arrays_by_descr
                .get(&Self::descriptor_is_recursive())
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Clear#is_recursive")?;
            <crate::components::ClearIsRecursive>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Clear#is_recursive")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Clear#is_recursive")?
        };
        Ok(Self { is_recursive })
    }
}

impl crate::AsComponents for Clear {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use crate::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.is_recursive as &dyn ComponentBatch)).map(|batch| {
                crate::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(Self::descriptor_is_recursive()),
                }
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl crate::ArchetypeReflectionMarker for Clear {}

impl Clear {
    /// Create a new `Clear`.
    #[inline]
    pub fn new(is_recursive: impl Into<crate::components::ClearIsRecursive>) -> Self {
        Self {
            is_recursive: is_recursive.into(),
        }
    }
}

impl ::re_byte_size::SizeBytes for Clear {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.is_recursive.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::ClearIsRecursive>::is_pod()
    }
}
