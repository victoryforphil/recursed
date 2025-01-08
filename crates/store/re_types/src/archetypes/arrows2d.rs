// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/arrows2d.fbs".

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

/// **Archetype**: 2D arrows with optional colors, radii, labels, etc.
///
/// ## Example
///
/// ### Simple batch of 2D arrows
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_arrow2d").spawn()?;
///
///     rec.log(
///         "arrows",
///         &rerun::Arrows2D::from_vectors([[1.0, 0.0], [0.0, -1.0], [-0.7, 0.7]])
///             .with_radii([0.025])
///             .with_origins([[0.25, 0.0], [0.25, 0.0], [-0.1, -0.1]])
///             .with_colors([[255, 0, 0], [0, 255, 0], [127, 0, 255]])
///             .with_labels(["right", "up", "left-down"]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/1200w.png">
///   <img src="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Arrows2D {
    /// All the vectors for each arrow in the batch.
    pub vectors: Vec<crate::components::Vector2D>,

    /// All the origin (base) positions for each arrow in the batch.
    ///
    /// If no origins are set, (0, 0) is used as the origin for each arrow.
    pub origins: Option<Vec<crate::components::Position2D>>,

    /// Optional radii for the arrows.
    ///
    /// The shaft is rendered as a line with `radius = 0.5 * radius`.
    /// The tip is rendered with `height = 2.0 * radius` and `radius = 1.0 * radius`.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the arrows.
    ///
    /// If there's a single label present, it will be placed at the center of the entity.
    /// Otherwise, each instance will have its own label.
    pub labels: Option<Vec<crate::components::Text>>,

    /// Optional choice of whether the text labels should be shown by default.
    pub show_labels: Option<crate::components::ShowLabels>,

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<crate::components::DrawOrder>,

    /// Optional class Ids for the points.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,
}

impl Arrows2D {
    /// Returns the [`ComponentDescriptor`] for [`Self::vectors`].
    #[inline]
    pub fn descriptor_vectors() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Vector2D".into(),
            archetype_field_name: Some("vectors".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::origins`].
    #[inline]
    pub fn descriptor_origins() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Position2D".into(),
            archetype_field_name: Some("origins".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::radii`].
    #[inline]
    pub fn descriptor_radii() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Radius".into(),
            archetype_field_name: Some("radii".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::colors`].
    #[inline]
    pub fn descriptor_colors() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("colors".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::labels`].
    #[inline]
    pub fn descriptor_labels() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Text".into(),
            archetype_field_name: Some("labels".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::show_labels`].
    #[inline]
    pub fn descriptor_show_labels() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.ShowLabels".into(),
            archetype_field_name: Some("show_labels".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::draw_order`].
    #[inline]
    pub fn descriptor_draw_order() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.DrawOrder".into(),
            archetype_field_name: Some("draw_order".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::class_ids`].
    #[inline]
    pub fn descriptor_class_ids() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.ClassId".into(),
            archetype_field_name: Some("class_ids".into()),
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Vector2D".into(),
            archetype_field_name: Some("vectors".into()),
        }]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Position2D".into(),
                archetype_field_name: Some("origins".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Arrows2DIndicator".into(),
                archetype_field_name: None,
            },
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Radius".into(),
                archetype_field_name: Some("radii".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("colors".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Text".into(),
                archetype_field_name: Some("labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.ShowLabels".into(),
                archetype_field_name: Some("show_labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.DrawOrder".into(),
                archetype_field_name: Some("draw_order".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.ClassId".into(),
                archetype_field_name: Some("class_ids".into()),
            },
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 9usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Vector2D".into(),
                archetype_field_name: Some("vectors".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Position2D".into(),
                archetype_field_name: Some("origins".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Arrows2DIndicator".into(),
                archetype_field_name: None,
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Radius".into(),
                archetype_field_name: Some("radii".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("colors".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.Text".into(),
                archetype_field_name: Some("labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.ShowLabels".into(),
                archetype_field_name: Some("show_labels".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.DrawOrder".into(),
                archetype_field_name: Some("draw_order".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                component_name: "rerun.components.ClassId".into(),
                archetype_field_name: Some("class_ids".into()),
            },
        ]
    });

impl Arrows2D {
    /// The total number of components in the archetype: 1 required, 2 recommended, 6 optional
    pub const NUM_COMPONENTS: usize = 9usize;
}

/// Indicator component for the [`Arrows2D`] [`::re_types_core::Archetype`]
pub type Arrows2DIndicator = ::re_types_core::GenericIndicatorComponent<Arrows2D>;

impl ::re_types_core::Archetype for Arrows2D {
    type Indicator = Arrows2DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Arrows2D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Arrows 2D"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: Arrows2DIndicator = Arrows2DIndicator::DEFAULT;
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
        let vectors = {
            let array = arrays_by_name
                .get("rerun.components.Vector2D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Arrows2D#vectors")?;
            <crate::components::Vector2D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Arrows2D#vectors")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Arrows2D#vectors")?
        };
        let origins = if let Some(array) = arrays_by_name.get("rerun.components.Position2D") {
            Some({
                <crate::components::Position2D>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows2D#origins")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows2D#origins")?
            })
        } else {
            None
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows2D#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows2D#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows2D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows2D#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows2D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows2D#labels")?
            })
        } else {
            None
        };
        let show_labels = if let Some(array) = arrays_by_name.get("rerun.components.ShowLabels") {
            <crate::components::ShowLabels>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Arrows2D#show_labels")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("rerun.components.DrawOrder") {
            <crate::components::DrawOrder>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Arrows2D#draw_order")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Arrows2D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Arrows2D#class_ids")?
            })
        } else {
            None
        };
        Ok(Self {
            vectors,
            origins,
            radii,
            colors,
            labels,
            show_labels,
            draw_order,
            class_ids,
        })
    }
}

impl ::re_types_core::AsComponents for Arrows2D {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.vectors as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                        archetype_field_name: Some(("vectors").into()),
                        component_name: ("rerun.components.Vector2D").into(),
                    }),
                }
            }),
            (self
                .origins
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                    archetype_field_name: Some(("origins").into()),
                    component_name: ("rerun.components.Position2D").into(),
                }),
            }),
            (self
                .radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                    archetype_field_name: Some(("radii").into()),
                    component_name: ("rerun.components.Radius").into(),
                }),
            }),
            (self
                .colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                    archetype_field_name: Some(("colors").into()),
                    component_name: ("rerun.components.Color").into(),
                }),
            }),
            (self
                .labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                    archetype_field_name: Some(("labels").into()),
                    component_name: ("rerun.components.Text").into(),
                }),
            }),
            (self
                .show_labels
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                    archetype_field_name: Some(("show_labels").into()),
                    component_name: ("rerun.components.ShowLabels").into(),
                }),
            }),
            (self
                .draw_order
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                    archetype_field_name: Some(("draw_order").into()),
                    component_name: ("rerun.components.DrawOrder").into(),
                }),
            }),
            (self
                .class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.Arrows2D".into()),
                    archetype_field_name: Some(("class_ids").into()),
                    component_name: ("rerun.components.ClassId").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Arrows2D {}

impl Arrows2D {
    /// Create a new `Arrows2D`.
    #[inline]
    pub(crate) fn new(
        vectors: impl IntoIterator<Item = impl Into<crate::components::Vector2D>>,
    ) -> Self {
        Self {
            vectors: vectors.into_iter().map(Into::into).collect(),
            origins: None,
            radii: None,
            colors: None,
            labels: None,
            show_labels: None,
            draw_order: None,
            class_ids: None,
        }
    }

    /// All the origin (base) positions for each arrow in the batch.
    ///
    /// If no origins are set, (0, 0) is used as the origin for each arrow.
    #[inline]
    pub fn with_origins(
        mut self,
        origins: impl IntoIterator<Item = impl Into<crate::components::Position2D>>,
    ) -> Self {
        self.origins = Some(origins.into_iter().map(Into::into).collect());
        self
    }

    /// Optional radii for the arrows.
    ///
    /// The shaft is rendered as a line with `radius = 0.5 * radius`.
    /// The tip is rendered with `height = 2.0 * radius` and `radius = 1.0 * radius`.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the points.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Optional text labels for the arrows.
    ///
    /// If there's a single label present, it will be placed at the center of the entity.
    /// Otherwise, each instance will have its own label.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    /// Optional choice of whether the text labels should be shown by default.
    #[inline]
    pub fn with_show_labels(
        mut self,
        show_labels: impl Into<crate::components::ShowLabels>,
    ) -> Self {
        self.show_labels = Some(show_labels.into());
        self
    }

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    #[inline]
    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }

    /// Optional class Ids for the points.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    #[inline]
    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }
}

impl ::re_byte_size::SizeBytes for Arrows2D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.vectors.heap_size_bytes()
            + self.origins.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.show_labels.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
            + self.class_ids.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::Vector2D>>::is_pod()
            && <Option<Vec<crate::components::Position2D>>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
            && <Option<Vec<crate::components::Text>>>::is_pod()
            && <Option<crate::components::ShowLabels>>::is_pod()
            && <Option<crate::components::DrawOrder>>::is_pod()
            && <Option<Vec<crate::components::ClassId>>>::is_pod()
    }
}
