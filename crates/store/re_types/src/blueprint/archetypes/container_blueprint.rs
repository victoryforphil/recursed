// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/container_blueprint.fbs".

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

/// **Archetype**: The description of a container.
#[derive(Clone, Debug)]
pub struct ContainerBlueprint {
    /// The class of the view.
    pub container_kind: crate::blueprint::components::ContainerKind,

    /// The name of the container.
    pub display_name: Option<crate::components::Name>,

    /// `ContainerId`s or `ViewId`s that are children of this container.
    pub contents: Option<Vec<crate::blueprint::components::IncludedContent>>,

    /// The layout shares of each column in the container.
    ///
    /// For [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal] containers, the length of this list should always match the number of contents.
    ///
    /// Ignored for [`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers.
    pub col_shares: Option<Vec<crate::blueprint::components::ColumnShare>>,

    /// The layout shares of each row of the container.
    ///
    /// For [`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers, the length of this list should always match the number of contents.
    ///
    /// Ignored for [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal] containers.
    pub row_shares: Option<Vec<crate::blueprint::components::RowShare>>,

    /// Which tab is active.
    ///
    /// Only applies to `Tabs` containers.
    pub active_tab: Option<crate::blueprint::components::ActiveTab>,

    /// Whether this container is visible.
    ///
    /// Defaults to true if not specified.
    pub visible: Option<crate::blueprint::components::Visible>,

    /// How many columns this grid should have.
    ///
    /// If unset, the grid layout will be auto.
    ///
    /// Ignored for [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal]/[`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers.
    pub grid_columns: Option<crate::blueprint::components::GridColumns>,
}

impl ContainerBlueprint {
    /// Returns the [`ComponentDescriptor`] for [`Self::container_kind`].
    #[inline]
    pub fn descriptor_container_kind() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.ContainerKind".into(),
            archetype_field_name: Some("container_kind".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::display_name`].
    #[inline]
    pub fn descriptor_display_name() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.components.Name".into(),
            archetype_field_name: Some("display_name".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::contents`].
    #[inline]
    pub fn descriptor_contents() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.IncludedContent".into(),
            archetype_field_name: Some("contents".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::col_shares`].
    #[inline]
    pub fn descriptor_col_shares() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.ColumnShare".into(),
            archetype_field_name: Some("col_shares".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::row_shares`].
    #[inline]
    pub fn descriptor_row_shares() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.RowShare".into(),
            archetype_field_name: Some("row_shares".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::active_tab`].
    #[inline]
    pub fn descriptor_active_tab() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.ActiveTab".into(),
            archetype_field_name: Some("active_tab".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::visible`].
    #[inline]
    pub fn descriptor_visible() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.Visible".into(),
            archetype_field_name: Some("visible".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::grid_columns`].
    #[inline]
    pub fn descriptor_grid_columns() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.GridColumns".into(),
            archetype_field_name: Some("grid_columns".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.ContainerBlueprintIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ContainerBlueprint::descriptor_container_kind()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ContainerBlueprint::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ContainerBlueprint::descriptor_display_name(),
            ContainerBlueprint::descriptor_contents(),
            ContainerBlueprint::descriptor_col_shares(),
            ContainerBlueprint::descriptor_row_shares(),
            ContainerBlueprint::descriptor_active_tab(),
            ContainerBlueprint::descriptor_visible(),
            ContainerBlueprint::descriptor_grid_columns(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 9usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ContainerBlueprint::descriptor_container_kind(),
            ContainerBlueprint::descriptor_indicator(),
            ContainerBlueprint::descriptor_display_name(),
            ContainerBlueprint::descriptor_contents(),
            ContainerBlueprint::descriptor_col_shares(),
            ContainerBlueprint::descriptor_row_shares(),
            ContainerBlueprint::descriptor_active_tab(),
            ContainerBlueprint::descriptor_visible(),
            ContainerBlueprint::descriptor_grid_columns(),
        ]
    });

impl ContainerBlueprint {
    /// The total number of components in the archetype: 1 required, 1 recommended, 7 optional
    pub const NUM_COMPONENTS: usize = 9usize;
}

/// Indicator component for the [`ContainerBlueprint`] [`::re_types_core::Archetype`]
pub type ContainerBlueprintIndicator =
    ::re_types_core::GenericIndicatorComponent<ContainerBlueprint>;

impl ::re_types_core::Archetype for ContainerBlueprint {
    type Indicator = ContainerBlueprintIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ContainerBlueprint".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Container blueprint"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: ContainerBlueprintIndicator = ContainerBlueprintIndicator::DEFAULT;
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
        let container_kind = {
            let array = arrays_by_descr
                .get(&Self::descriptor_container_kind())
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#container_kind")?;
            <crate::blueprint::components::ContainerKind>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#container_kind")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#container_kind")?
        };
        let display_name =
            if let Some(array) = arrays_by_descr.get(&Self::descriptor_display_name()) {
                <crate::components::Name>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#display_name")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let contents = if let Some(array) = arrays_by_descr.get(&Self::descriptor_contents()) {
            Some({
                <crate::blueprint::components::IncludedContent>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#contents")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#contents")?
            })
        } else {
            None
        };
        let col_shares = if let Some(array) = arrays_by_descr.get(&Self::descriptor_col_shares()) {
            Some({
                <crate::blueprint::components::ColumnShare>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#col_shares")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#col_shares")?
            })
        } else {
            None
        };
        let row_shares = if let Some(array) = arrays_by_descr.get(&Self::descriptor_row_shares()) {
            Some({
                <crate::blueprint::components::RowShare>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#row_shares")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#row_shares")?
            })
        } else {
            None
        };
        let active_tab = if let Some(array) = arrays_by_descr.get(&Self::descriptor_active_tab()) {
            <crate::blueprint::components::ActiveTab>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#active_tab")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let visible = if let Some(array) = arrays_by_descr.get(&Self::descriptor_visible()) {
            <crate::blueprint::components::Visible>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#visible")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let grid_columns =
            if let Some(array) = arrays_by_descr.get(&Self::descriptor_grid_columns()) {
                <crate::blueprint::components::GridColumns>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#grid_columns")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        Ok(Self {
            container_kind,
            display_name,
            contents,
            col_shares,
            row_shares,
            active_tab,
            visible,
            grid_columns,
        })
    }
}

impl ::re_types_core::AsComponents for ContainerBlueprint {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.container_kind as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(Self::descriptor_container_kind()),
                }
            }),
            (self
                .display_name
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_display_name()),
            }),
            (self
                .contents
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_contents()),
            }),
            (self
                .col_shares
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_col_shares()),
            }),
            (self
                .row_shares
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_row_shares()),
            }),
            (self
                .active_tab
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_active_tab()),
            }),
            (self
                .visible
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_visible()),
            }),
            (self
                .grid_columns
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_grid_columns()),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ContainerBlueprint {}

impl ContainerBlueprint {
    /// Create a new `ContainerBlueprint`.
    #[inline]
    pub fn new(container_kind: impl Into<crate::blueprint::components::ContainerKind>) -> Self {
        Self {
            container_kind: container_kind.into(),
            display_name: None,
            contents: None,
            col_shares: None,
            row_shares: None,
            active_tab: None,
            visible: None,
            grid_columns: None,
        }
    }

    /// The name of the container.
    #[inline]
    pub fn with_display_name(mut self, display_name: impl Into<crate::components::Name>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    /// `ContainerId`s or `ViewId`s that are children of this container.
    #[inline]
    pub fn with_contents(
        mut self,
        contents: impl IntoIterator<Item = impl Into<crate::blueprint::components::IncludedContent>>,
    ) -> Self {
        self.contents = Some(contents.into_iter().map(Into::into).collect());
        self
    }

    /// The layout shares of each column in the container.
    ///
    /// For [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal] containers, the length of this list should always match the number of contents.
    ///
    /// Ignored for [`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers.
    #[inline]
    pub fn with_col_shares(
        mut self,
        col_shares: impl IntoIterator<Item = impl Into<crate::blueprint::components::ColumnShare>>,
    ) -> Self {
        self.col_shares = Some(col_shares.into_iter().map(Into::into).collect());
        self
    }

    /// The layout shares of each row of the container.
    ///
    /// For [`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers, the length of this list should always match the number of contents.
    ///
    /// Ignored for [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal] containers.
    #[inline]
    pub fn with_row_shares(
        mut self,
        row_shares: impl IntoIterator<Item = impl Into<crate::blueprint::components::RowShare>>,
    ) -> Self {
        self.row_shares = Some(row_shares.into_iter().map(Into::into).collect());
        self
    }

    /// Which tab is active.
    ///
    /// Only applies to `Tabs` containers.
    #[inline]
    pub fn with_active_tab(
        mut self,
        active_tab: impl Into<crate::blueprint::components::ActiveTab>,
    ) -> Self {
        self.active_tab = Some(active_tab.into());
        self
    }

    /// Whether this container is visible.
    ///
    /// Defaults to true if not specified.
    #[inline]
    pub fn with_visible(
        mut self,
        visible: impl Into<crate::blueprint::components::Visible>,
    ) -> Self {
        self.visible = Some(visible.into());
        self
    }

    /// How many columns this grid should have.
    ///
    /// If unset, the grid layout will be auto.
    ///
    /// Ignored for [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal]/[`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers.
    #[inline]
    pub fn with_grid_columns(
        mut self,
        grid_columns: impl Into<crate::blueprint::components::GridColumns>,
    ) -> Self {
        self.grid_columns = Some(grid_columns.into());
        self
    }
}

impl ::re_byte_size::SizeBytes for ContainerBlueprint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.container_kind.heap_size_bytes()
            + self.display_name.heap_size_bytes()
            + self.contents.heap_size_bytes()
            + self.col_shares.heap_size_bytes()
            + self.row_shares.heap_size_bytes()
            + self.active_tab.heap_size_bytes()
            + self.visible.heap_size_bytes()
            + self.grid_columns.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::blueprint::components::ContainerKind>::is_pod()
            && <Option<crate::components::Name>>::is_pod()
            && <Option<Vec<crate::blueprint::components::IncludedContent>>>::is_pod()
            && <Option<Vec<crate::blueprint::components::ColumnShare>>>::is_pod()
            && <Option<Vec<crate::blueprint::components::RowShare>>>::is_pod()
            && <Option<crate::blueprint::components::ActiveTab>>::is_pod()
            && <Option<crate::blueprint::components::Visible>>::is_pod()
            && <Option<crate::blueprint::components::GridColumns>>::is_pod()
    }
}
