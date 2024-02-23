// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/audio.fbs".

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

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Raw uncompressed PCM-encoded audio data (e.g. WAV, AIFF, etc.).
#[derive(Clone, Debug, PartialEq)]
pub struct Audio {
    /// The audio data.
    ///
    /// Either a single-channel mono vector, or a 2xN matrix for stereo.
    pub data: crate::components::TensorData,

    /// Sample-rate of the audio data, in Hz.
    ///
    /// Commonly 44100 Hz (default) or 48000 Hz.
    pub sample_rate: Option<crate::components::AudioSampleRate>,
}

impl ::re_types_core::SizeBytes for Audio {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.data.heap_size_bytes() + self.sample_rate.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::TensorData>::is_pod()
            && <Option<crate::components::AudioSampleRate>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.TensorData".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.AudioSampleRate".into(),
            "rerun.components.AudioIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.TensorData".into(),
            "rerun.components.AudioSampleRate".into(),
            "rerun.components.AudioIndicator".into(),
        ]
    });

impl Audio {
    /// The total number of components in the archetype: 1 required, 2 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`Audio`] [`::re_types_core::Archetype`]
pub type AudioIndicator = ::re_types_core::GenericIndicatorComponent<Audio>;

impl ::re_types_core::Archetype for Audio {
    type Indicator = AudioIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Audio".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Audio"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: AudioIndicator = AudioIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let data = {
            let array = arrays_by_name
                .get("rerun.components.TensorData")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Audio#data")?;
            <crate::components::TensorData>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Audio#data")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Audio#data")?
        };
        let sample_rate =
            if let Some(array) = arrays_by_name.get("rerun.components.AudioSampleRate") {
                <crate::components::AudioSampleRate>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Audio#sample_rate")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        Ok(Self { data, sample_rate })
    }
}

impl ::re_types_core::AsComponents for Audio {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.data as &dyn ComponentBatch).into()),
            self.sample_rate
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Audio {}

impl Audio {
    /// Create a new `Audio`.
    #[inline]
    pub fn new(data: impl Into<crate::components::TensorData>) -> Self {
        Self {
            data: data.into(),
            sample_rate: None,
        }
    }

    /// Sample-rate of the audio data, in Hz.
    ///
    /// Commonly 44100 Hz (default) or 48000 Hz.
    #[inline]
    pub fn with_sample_rate(
        mut self,
        sample_rate: impl Into<crate::components::AudioSampleRate>,
    ) -> Self {
        self.sample_rate = Some(sample_rate.into());
        self
    }
}
