// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/transform3d.fbs".

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

/// **Archetype**: A transform between two 3D spaces, i.e. a pose.
///
/// From the point of view of the entity's coordinate system,
/// all components are applied in the inverse order they are listed here.
/// E.g. if both a translation and a max3x3 transform are present,
/// the 3x3 matrix is applied first, followed by the translation.
///
/// Whenever you log this archetype, it will write all components, even if you do not explicitly set them.
/// This means that if you first log a transform with only a translation, and then log one with only a rotation,
/// it will be resolved to a transform with only a rotation.
///
/// For transforms that affect only a single entity and do not propagate along the entity tree refer to [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D].
///
/// ## Examples
///
/// ### Variety of 3D transforms
/// ```ignore
/// use std::f32::consts::TAU;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_transform3d").spawn()?;
///
///     let arrow = rerun::Arrows3D::from_vectors([(0.0, 1.0, 0.0)]).with_origins([(0.0, 0.0, 0.0)]);
///
///     rec.log("base", &arrow)?;
///
///     rec.log(
///         "base/translated",
///         &rerun::Transform3D::from_translation([1.0, 0.0, 0.0]),
///     )?;
///
///     rec.log("base/translated", &arrow)?;
///
///     rec.log(
///         "base/rotated_scaled",
///         &rerun::Transform3D::from_rotation_scale(
///             rerun::RotationAxisAngle::new([0.0, 0.0, 1.0], rerun::Angle::from_radians(TAU / 8.0)),
///             rerun::Scale3D::from(2.0),
///         ),
///     )?;
///
///     rec.log("base/rotated_scaled", &arrow)?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/1200w.png">
///   <img src="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/full.png" width="640">
/// </picture>
/// </center>
///
/// ### Transform hierarchy
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_transform3d_hierarchy").spawn()?;
///
///     // TODO(#5521): log two views as in the python example
///
///     rec.set_time_seconds("sim_time", 0.0);
///
///     // Planetary motion is typically in the XY plane.
///     rec.log_static("/", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP)?;
///
///     // Setup points, all are in the center of their own space:
///     rec.log(
///         "sun",
///         &rerun::Points3D::new([[0.0, 0.0, 0.0]])
///             .with_radii([1.0])
///             .with_colors([rerun::Color::from_rgb(255, 200, 10)]),
///     )?;
///     rec.log(
///         "sun/planet",
///         &rerun::Points3D::new([[0.0, 0.0, 0.0]])
///             .with_radii([0.4])
///             .with_colors([rerun::Color::from_rgb(40, 80, 200)]),
///     )?;
///     rec.log(
///         "sun/planet/moon",
///         &rerun::Points3D::new([[0.0, 0.0, 0.0]])
///             .with_radii([0.15])
///             .with_colors([rerun::Color::from_rgb(180, 180, 180)]),
///     )?;
///
///     // Draw fixed paths where the planet & moon move.
///     let d_planet = 6.0;
///     let d_moon = 3.0;
///     let angles = (0..=100).map(|i| i as f32 * 0.01 * std::f32::consts::TAU);
///     let circle: Vec<_> = angles.map(|angle| [angle.sin(), angle.cos()]).collect();
///     rec.log(
///         "sun/planet_path",
///         &rerun::LineStrips3D::new([rerun::LineStrip3D::from_iter(
///             circle
///                 .iter()
///                 .map(|p| [p[0] * d_planet, p[1] * d_planet, 0.0]),
///         )]),
///     )?;
///     rec.log(
///         "sun/planet/moon_path",
///         &rerun::LineStrips3D::new([rerun::LineStrip3D::from_iter(
///             circle.iter().map(|p| [p[0] * d_moon, p[1] * d_moon, 0.0]),
///         )]),
///     )?;
///
///     // Movement via transforms.
///     for i in 0..(6 * 120) {
///         let time = i as f32 / 120.0;
///         rec.set_time_seconds("sim_time", time);
///         let r_moon = time * 5.0;
///         let r_planet = time * 2.0;
///
///         rec.log(
///             "sun/planet",
///             &rerun::Transform3D::from_translation_rotation(
///                 [r_planet.sin() * d_planet, r_planet.cos() * d_planet, 0.0],
///                 rerun::RotationAxisAngle {
///                     axis: [1.0, 0.0, 0.0].into(),
///                     angle: rerun::Angle::from_degrees(20.0),
///                 },
///             ),
///         )?;
///         rec.log(
///             "sun/planet/moon",
///             &rerun::Transform3D::from_translation([
///                 r_moon.cos() * d_moon,
///                 r_moon.sin() * d_moon,
///                 0.0,
///             ])
///             .with_relation(rerun::TransformRelation::ChildFromParent),
///         )?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/1200w.png">
///   <img src="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Transform3D {
    /// Translation vector.
    pub translation: Option<SerializedComponentBatch>,

    /// Rotation via axis + angle.
    pub rotation_axis_angle: Option<SerializedComponentBatch>,

    /// Rotation via quaternion.
    pub quaternion: Option<SerializedComponentBatch>,

    /// Scaling factor.
    pub scale: Option<SerializedComponentBatch>,

    /// 3x3 transformation matrix.
    pub mat3x3: Option<SerializedComponentBatch>,

    /// Specifies the relation this transform establishes between this entity and its parent.
    pub relation: Option<SerializedComponentBatch>,

    /// Visual length of the 3 axes.
    ///
    /// The length is interpreted in the local coordinate system of the transform.
    /// If the transform is scaled, the axes will be scaled accordingly.
    pub axis_length: Option<SerializedComponentBatch>,
}

impl Transform3D {
    /// Returns the [`ComponentDescriptor`] for [`Self::translation`].
    #[inline]
    pub fn descriptor_translation() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Transform3D".into()),
            component_name: "rerun.components.Translation3D".into(),
            archetype_field_name: Some("translation".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::rotation_axis_angle`].
    #[inline]
    pub fn descriptor_rotation_axis_angle() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Transform3D".into()),
            component_name: "rerun.components.RotationAxisAngle".into(),
            archetype_field_name: Some("rotation_axis_angle".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::quaternion`].
    #[inline]
    pub fn descriptor_quaternion() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Transform3D".into()),
            component_name: "rerun.components.RotationQuat".into(),
            archetype_field_name: Some("quaternion".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::scale`].
    #[inline]
    pub fn descriptor_scale() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Transform3D".into()),
            component_name: "rerun.components.Scale3D".into(),
            archetype_field_name: Some("scale".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::mat3x3`].
    #[inline]
    pub fn descriptor_mat3x3() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Transform3D".into()),
            component_name: "rerun.components.TransformMat3x3".into(),
            archetype_field_name: Some("mat3x3".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::relation`].
    #[inline]
    pub fn descriptor_relation() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Transform3D".into()),
            component_name: "rerun.components.TransformRelation".into(),
            archetype_field_name: Some("relation".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::axis_length`].
    #[inline]
    pub fn descriptor_axis_length() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Transform3D".into()),
            component_name: "rerun.components.AxisLength".into(),
            archetype_field_name: Some("axis_length".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Transform3D".into()),
            component_name: "rerun.components.Transform3DIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Transform3D::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Transform3D::descriptor_translation(),
            Transform3D::descriptor_rotation_axis_angle(),
            Transform3D::descriptor_quaternion(),
            Transform3D::descriptor_scale(),
            Transform3D::descriptor_mat3x3(),
            Transform3D::descriptor_relation(),
            Transform3D::descriptor_axis_length(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 8usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Transform3D::descriptor_indicator(),
            Transform3D::descriptor_translation(),
            Transform3D::descriptor_rotation_axis_angle(),
            Transform3D::descriptor_quaternion(),
            Transform3D::descriptor_scale(),
            Transform3D::descriptor_mat3x3(),
            Transform3D::descriptor_relation(),
            Transform3D::descriptor_axis_length(),
        ]
    });

impl Transform3D {
    /// The total number of components in the archetype: 0 required, 1 recommended, 7 optional
    pub const NUM_COMPONENTS: usize = 8usize;
}

/// Indicator component for the [`Transform3D`] [`::re_types_core::Archetype`]
pub type Transform3DIndicator = ::re_types_core::GenericIndicatorComponent<Transform3D>;

impl ::re_types_core::Archetype for Transform3D {
    type Indicator = Transform3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Transform3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Transform 3D"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: Transform3DIndicator = Transform3DIndicator::DEFAULT;
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
        let translation = arrays_by_descr
            .get(&Self::descriptor_translation())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_translation())
            });
        let rotation_axis_angle = arrays_by_descr
            .get(&Self::descriptor_rotation_axis_angle())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_rotation_axis_angle())
            });
        let quaternion = arrays_by_descr
            .get(&Self::descriptor_quaternion())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_quaternion())
            });
        let scale = arrays_by_descr
            .get(&Self::descriptor_scale())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_scale()));
        let mat3x3 = arrays_by_descr
            .get(&Self::descriptor_mat3x3())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_mat3x3()));
        let relation = arrays_by_descr
            .get(&Self::descriptor_relation())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_relation()));
        let axis_length = arrays_by_descr
            .get(&Self::descriptor_axis_length())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_axis_length())
            });
        Ok(Self {
            translation,
            rotation_axis_angle,
            quaternion,
            scale,
            mat3x3,
            relation,
            axis_length,
        })
    }
}

impl ::re_types_core::AsComponents for Transform3D {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Self::indicator().serialized(),
            self.translation.clone(),
            self.rotation_axis_angle.clone(),
            self.quaternion.clone(),
            self.scale.clone(),
            self.mat3x3.clone(),
            self.relation.clone(),
            self.axis_length.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Transform3D {}

impl Transform3D {
    /// Update only some specific fields of a `Transform3D`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `Transform3D`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            translation: Some(SerializedComponentBatch::new(
                crate::components::Translation3D::arrow_empty(),
                Self::descriptor_translation(),
            )),
            rotation_axis_angle: Some(SerializedComponentBatch::new(
                crate::components::RotationAxisAngle::arrow_empty(),
                Self::descriptor_rotation_axis_angle(),
            )),
            quaternion: Some(SerializedComponentBatch::new(
                crate::components::RotationQuat::arrow_empty(),
                Self::descriptor_quaternion(),
            )),
            scale: Some(SerializedComponentBatch::new(
                crate::components::Scale3D::arrow_empty(),
                Self::descriptor_scale(),
            )),
            mat3x3: Some(SerializedComponentBatch::new(
                crate::components::TransformMat3x3::arrow_empty(),
                Self::descriptor_mat3x3(),
            )),
            relation: Some(SerializedComponentBatch::new(
                crate::components::TransformRelation::arrow_empty(),
                Self::descriptor_relation(),
            )),
            axis_length: Some(SerializedComponentBatch::new(
                crate::components::AxisLength::arrow_empty(),
                Self::descriptor_axis_length(),
            )),
        }
    }

    /// Translation vector.
    #[inline]
    pub fn with_translation(
        mut self,
        translation: impl Into<crate::components::Translation3D>,
    ) -> Self {
        self.translation = try_serialize_field(Self::descriptor_translation(), [translation]);
        self
    }

    /// Rotation via axis + angle.
    #[inline]
    pub fn with_rotation_axis_angle(
        mut self,
        rotation_axis_angle: impl Into<crate::components::RotationAxisAngle>,
    ) -> Self {
        self.rotation_axis_angle = try_serialize_field(
            Self::descriptor_rotation_axis_angle(),
            [rotation_axis_angle],
        );
        self
    }

    /// Rotation via quaternion.
    #[inline]
    pub fn with_quaternion(
        mut self,
        quaternion: impl Into<crate::components::RotationQuat>,
    ) -> Self {
        self.quaternion = try_serialize_field(Self::descriptor_quaternion(), [quaternion]);
        self
    }

    /// Scaling factor.
    #[inline]
    pub fn with_scale(mut self, scale: impl Into<crate::components::Scale3D>) -> Self {
        self.scale = try_serialize_field(Self::descriptor_scale(), [scale]);
        self
    }

    /// 3x3 transformation matrix.
    #[inline]
    pub fn with_mat3x3(mut self, mat3x3: impl Into<crate::components::TransformMat3x3>) -> Self {
        self.mat3x3 = try_serialize_field(Self::descriptor_mat3x3(), [mat3x3]);
        self
    }

    /// Specifies the relation this transform establishes between this entity and its parent.
    #[inline]
    pub fn with_relation(
        mut self,
        relation: impl Into<crate::components::TransformRelation>,
    ) -> Self {
        self.relation = try_serialize_field(Self::descriptor_relation(), [relation]);
        self
    }

    /// Visual length of the 3 axes.
    ///
    /// The length is interpreted in the local coordinate system of the transform.
    /// If the transform is scaled, the axes will be scaled accordingly.
    #[inline]
    pub fn with_axis_length(
        mut self,
        axis_length: impl Into<crate::components::AxisLength>,
    ) -> Self {
        self.axis_length = try_serialize_field(Self::descriptor_axis_length(), [axis_length]);
        self
    }
}

impl ::re_byte_size::SizeBytes for Transform3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.translation.heap_size_bytes()
            + self.rotation_axis_angle.heap_size_bytes()
            + self.quaternion.heap_size_bytes()
            + self.scale.heap_size_bytes()
            + self.mat3x3.heap_size_bytes()
            + self.relation.heap_size_bytes()
            + self.axis_length.heap_size_bytes()
    }
}
