// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/force_position.fbs".

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

use ::re_types_core::external::arrow;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Similar to gravity, this force pulls nodes towards a specific position.
#[derive(Clone, Debug)]
pub struct ForcePosition {
    /// Whether the position force is enabled.
    ///
    /// The position force pulls nodes towards a specific position, similar to gravity.
    pub enabled: Option<crate::blueprint::components::Enabled>,

    /// The strength of the force.
    pub strength: Option<crate::blueprint::components::ForceStrength>,

    /// The position where the nodes should be pulled towards.
    pub position: Option<crate::components::Position2D>,
}

impl ForcePosition {
    /// Returns the [`ComponentDescriptor`] for [`Self::enabled`].
    #[inline]
    pub fn descriptor_enabled() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ForcePosition".into()),
            component_name: "rerun.blueprint.components.Enabled".into(),
            archetype_field_name: Some("enabled".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::strength`].
    #[inline]
    pub fn descriptor_strength() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ForcePosition".into()),
            component_name: "rerun.blueprint.components.ForceStrength".into(),
            archetype_field_name: Some("strength".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::position`].
    #[inline]
    pub fn descriptor_position() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ForcePosition".into()),
            component_name: "rerun.components.Position2D".into(),
            archetype_field_name: Some("position".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ForcePosition".into()),
            component_name: "rerun.blueprint.components.ForcePositionIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ForcePosition::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ForcePosition::descriptor_enabled(),
            ForcePosition::descriptor_strength(),
            ForcePosition::descriptor_position(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ForcePosition::descriptor_indicator(),
            ForcePosition::descriptor_enabled(),
            ForcePosition::descriptor_strength(),
            ForcePosition::descriptor_position(),
        ]
    });

impl ForcePosition {
    /// The total number of components in the archetype: 0 required, 1 recommended, 3 optional
    pub const NUM_COMPONENTS: usize = 4usize;
}

/// Indicator component for the [`ForcePosition`] [`::re_types_core::Archetype`]
pub type ForcePositionIndicator = ::re_types_core::GenericIndicatorComponent<ForcePosition>;

impl ::re_types_core::Archetype for ForcePosition {
    type Indicator = ForcePositionIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ForcePosition".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Force position"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: ForcePositionIndicator = ForcePositionIndicator::DEFAULT;
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
        arrow_data: impl IntoIterator<Item = (ComponentName, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let enabled = if let Some(array) = arrays_by_name.get("rerun.blueprint.components.Enabled")
        {
            <crate::blueprint::components::Enabled>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ForcePosition#enabled")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let strength =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.ForceStrength") {
                <crate::blueprint::components::ForceStrength>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ForcePosition#strength")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let position = if let Some(array) = arrays_by_name.get("rerun.components.Position2D") {
            <crate::components::Position2D>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ForcePosition#position")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            enabled,
            strength,
            position,
        })
    }
}

impl ::re_types_core::AsComponents for ForcePosition {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (self
                .enabled
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_enabled()),
            }),
            (self
                .strength
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_strength()),
            }),
            (self
                .position
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_position()),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ForcePosition {}

impl ForcePosition {
    /// Create a new `ForcePosition`.
    #[inline]
    pub fn new() -> Self {
        Self {
            enabled: None,
            strength: None,
            position: None,
        }
    }

    /// Whether the position force is enabled.
    ///
    /// The position force pulls nodes towards a specific position, similar to gravity.
    #[inline]
    pub fn with_enabled(
        mut self,
        enabled: impl Into<crate::blueprint::components::Enabled>,
    ) -> Self {
        self.enabled = Some(enabled.into());
        self
    }

    /// The strength of the force.
    #[inline]
    pub fn with_strength(
        mut self,
        strength: impl Into<crate::blueprint::components::ForceStrength>,
    ) -> Self {
        self.strength = Some(strength.into());
        self
    }

    /// The position where the nodes should be pulled towards.
    #[inline]
    pub fn with_position(mut self, position: impl Into<crate::components::Position2D>) -> Self {
        self.position = Some(position.into());
        self
    }
}

impl ::re_byte_size::SizeBytes for ForcePosition {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.enabled.heap_size_bytes()
            + self.strength.heap_size_bytes()
            + self.position.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::blueprint::components::Enabled>>::is_pod()
            && <Option<crate::blueprint::components::ForceStrength>>::is_pod()
            && <Option<crate::components::Position2D>>::is_pod()
    }
}
