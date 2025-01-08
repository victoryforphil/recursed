// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

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

#[derive(Clone, Debug, PartialEq)]
pub struct AffixFuzzer3 {
    pub fuzz2001: Option<crate::testing::components::AffixFuzzer1>,
    pub fuzz2002: Option<crate::testing::components::AffixFuzzer2>,
    pub fuzz2003: Option<crate::testing::components::AffixFuzzer3>,
    pub fuzz2004: Option<crate::testing::components::AffixFuzzer4>,
    pub fuzz2005: Option<crate::testing::components::AffixFuzzer5>,
    pub fuzz2006: Option<crate::testing::components::AffixFuzzer6>,
    pub fuzz2007: Option<crate::testing::components::AffixFuzzer7>,
    pub fuzz2008: Option<crate::testing::components::AffixFuzzer8>,
    pub fuzz2009: Option<crate::testing::components::AffixFuzzer9>,
    pub fuzz2010: Option<crate::testing::components::AffixFuzzer10>,
    pub fuzz2011: Option<crate::testing::components::AffixFuzzer11>,
    pub fuzz2012: Option<crate::testing::components::AffixFuzzer12>,
    pub fuzz2013: Option<crate::testing::components::AffixFuzzer13>,
    pub fuzz2014: Option<crate::testing::components::AffixFuzzer14>,
    pub fuzz2015: Option<crate::testing::components::AffixFuzzer15>,
    pub fuzz2016: Option<crate::testing::components::AffixFuzzer16>,
    pub fuzz2017: Option<crate::testing::components::AffixFuzzer17>,
    pub fuzz2018: Option<crate::testing::components::AffixFuzzer18>,
}

impl AffixFuzzer3 {
    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2001`].
    #[inline]
    pub fn descriptor_fuzz2001() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer1".into(),
            archetype_field_name: Some("fuzz2001".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2002`].
    #[inline]
    pub fn descriptor_fuzz2002() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer2".into(),
            archetype_field_name: Some("fuzz2002".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2003`].
    #[inline]
    pub fn descriptor_fuzz2003() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer3".into(),
            archetype_field_name: Some("fuzz2003".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2004`].
    #[inline]
    pub fn descriptor_fuzz2004() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer4".into(),
            archetype_field_name: Some("fuzz2004".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2005`].
    #[inline]
    pub fn descriptor_fuzz2005() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer5".into(),
            archetype_field_name: Some("fuzz2005".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2006`].
    #[inline]
    pub fn descriptor_fuzz2006() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer6".into(),
            archetype_field_name: Some("fuzz2006".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2007`].
    #[inline]
    pub fn descriptor_fuzz2007() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer7".into(),
            archetype_field_name: Some("fuzz2007".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2008`].
    #[inline]
    pub fn descriptor_fuzz2008() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer8".into(),
            archetype_field_name: Some("fuzz2008".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2009`].
    #[inline]
    pub fn descriptor_fuzz2009() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer9".into(),
            archetype_field_name: Some("fuzz2009".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2010`].
    #[inline]
    pub fn descriptor_fuzz2010() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer10".into(),
            archetype_field_name: Some("fuzz2010".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2011`].
    #[inline]
    pub fn descriptor_fuzz2011() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer11".into(),
            archetype_field_name: Some("fuzz2011".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2012`].
    #[inline]
    pub fn descriptor_fuzz2012() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer12".into(),
            archetype_field_name: Some("fuzz2012".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2013`].
    #[inline]
    pub fn descriptor_fuzz2013() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer13".into(),
            archetype_field_name: Some("fuzz2013".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2014`].
    #[inline]
    pub fn descriptor_fuzz2014() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer14".into(),
            archetype_field_name: Some("fuzz2014".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2015`].
    #[inline]
    pub fn descriptor_fuzz2015() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer15".into(),
            archetype_field_name: Some("fuzz2015".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2016`].
    #[inline]
    pub fn descriptor_fuzz2016() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer16".into(),
            archetype_field_name: Some("fuzz2016".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2017`].
    #[inline]
    pub fn descriptor_fuzz2017() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer17".into(),
            archetype_field_name: Some("fuzz2017".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::fuzz2018`].
    #[inline]
    pub fn descriptor_fuzz2018() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer18".into(),
            archetype_field_name: Some("fuzz2018".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.testing.archetypes.AffixFuzzer3".into()),
            component_name: "rerun.testing.components.AffixFuzzer3Indicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [AffixFuzzer3::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 18usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            AffixFuzzer3::descriptor_fuzz2001(),
            AffixFuzzer3::descriptor_fuzz2002(),
            AffixFuzzer3::descriptor_fuzz2003(),
            AffixFuzzer3::descriptor_fuzz2004(),
            AffixFuzzer3::descriptor_fuzz2005(),
            AffixFuzzer3::descriptor_fuzz2006(),
            AffixFuzzer3::descriptor_fuzz2007(),
            AffixFuzzer3::descriptor_fuzz2008(),
            AffixFuzzer3::descriptor_fuzz2009(),
            AffixFuzzer3::descriptor_fuzz2010(),
            AffixFuzzer3::descriptor_fuzz2011(),
            AffixFuzzer3::descriptor_fuzz2012(),
            AffixFuzzer3::descriptor_fuzz2013(),
            AffixFuzzer3::descriptor_fuzz2014(),
            AffixFuzzer3::descriptor_fuzz2015(),
            AffixFuzzer3::descriptor_fuzz2016(),
            AffixFuzzer3::descriptor_fuzz2017(),
            AffixFuzzer3::descriptor_fuzz2018(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 19usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            AffixFuzzer3::descriptor_indicator(),
            AffixFuzzer3::descriptor_fuzz2001(),
            AffixFuzzer3::descriptor_fuzz2002(),
            AffixFuzzer3::descriptor_fuzz2003(),
            AffixFuzzer3::descriptor_fuzz2004(),
            AffixFuzzer3::descriptor_fuzz2005(),
            AffixFuzzer3::descriptor_fuzz2006(),
            AffixFuzzer3::descriptor_fuzz2007(),
            AffixFuzzer3::descriptor_fuzz2008(),
            AffixFuzzer3::descriptor_fuzz2009(),
            AffixFuzzer3::descriptor_fuzz2010(),
            AffixFuzzer3::descriptor_fuzz2011(),
            AffixFuzzer3::descriptor_fuzz2012(),
            AffixFuzzer3::descriptor_fuzz2013(),
            AffixFuzzer3::descriptor_fuzz2014(),
            AffixFuzzer3::descriptor_fuzz2015(),
            AffixFuzzer3::descriptor_fuzz2016(),
            AffixFuzzer3::descriptor_fuzz2017(),
            AffixFuzzer3::descriptor_fuzz2018(),
        ]
    });

impl AffixFuzzer3 {
    /// The total number of components in the archetype: 0 required, 1 recommended, 18 optional
    pub const NUM_COMPONENTS: usize = 19usize;
}

/// Indicator component for the [`AffixFuzzer3`] [`::re_types_core::Archetype`]
pub type AffixFuzzer3Indicator = ::re_types_core::GenericIndicatorComponent<AffixFuzzer3>;

impl ::re_types_core::Archetype for AffixFuzzer3 {
    type Indicator = AffixFuzzer3Indicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.testing.archetypes.AffixFuzzer3".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Affix fuzzer 3"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: AffixFuzzer3Indicator = AffixFuzzer3Indicator::DEFAULT;
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
        let fuzz2001 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2001()) {
            <crate::testing::components::AffixFuzzer1>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2001")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2002 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2002()) {
            <crate::testing::components::AffixFuzzer2>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2002")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2003 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2003()) {
            <crate::testing::components::AffixFuzzer3>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2003")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2004 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2004()) {
            <crate::testing::components::AffixFuzzer4>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2004")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2005 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2005()) {
            <crate::testing::components::AffixFuzzer5>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2005")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2006 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2006()) {
            <crate::testing::components::AffixFuzzer6>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2006")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2007 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2007()) {
            <crate::testing::components::AffixFuzzer7>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2007")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2008 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2008()) {
            <crate::testing::components::AffixFuzzer8>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2008")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2009 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2009()) {
            <crate::testing::components::AffixFuzzer9>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2009")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2010 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2010()) {
            <crate::testing::components::AffixFuzzer10>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2010")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2011 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2011()) {
            <crate::testing::components::AffixFuzzer11>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2011")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2012 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2012()) {
            <crate::testing::components::AffixFuzzer12>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2012")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2013 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2013()) {
            <crate::testing::components::AffixFuzzer13>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2013")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2014 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2014()) {
            <crate::testing::components::AffixFuzzer14>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2014")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2015 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2015()) {
            <crate::testing::components::AffixFuzzer15>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2015")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2016 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2016()) {
            <crate::testing::components::AffixFuzzer16>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2016")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2017 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2017()) {
            <crate::testing::components::AffixFuzzer17>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2017")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let fuzz2018 = if let Some(array) = arrays_by_descr.get(&Self::descriptor_fuzz2018()) {
            <crate::testing::components::AffixFuzzer18>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2018")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            fuzz2001,
            fuzz2002,
            fuzz2003,
            fuzz2004,
            fuzz2005,
            fuzz2006,
            fuzz2007,
            fuzz2008,
            fuzz2009,
            fuzz2010,
            fuzz2011,
            fuzz2012,
            fuzz2013,
            fuzz2014,
            fuzz2015,
            fuzz2016,
            fuzz2017,
            fuzz2018,
        })
    }
}

impl ::re_types_core::AsComponents for AffixFuzzer3 {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (self
                .fuzz2001
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2001()),
            }),
            (self
                .fuzz2002
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2002()),
            }),
            (self
                .fuzz2003
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2003()),
            }),
            (self
                .fuzz2004
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2004()),
            }),
            (self
                .fuzz2005
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2005()),
            }),
            (self
                .fuzz2006
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2006()),
            }),
            (self
                .fuzz2007
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2007()),
            }),
            (self
                .fuzz2008
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2008()),
            }),
            (self
                .fuzz2009
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2009()),
            }),
            (self
                .fuzz2010
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2010()),
            }),
            (self
                .fuzz2011
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2011()),
            }),
            (self
                .fuzz2012
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2012()),
            }),
            (self
                .fuzz2013
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2013()),
            }),
            (self
                .fuzz2014
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2014()),
            }),
            (self
                .fuzz2015
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2015()),
            }),
            (self
                .fuzz2016
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2016()),
            }),
            (self
                .fuzz2017
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2017()),
            }),
            (self
                .fuzz2018
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_fuzz2018()),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for AffixFuzzer3 {}

impl AffixFuzzer3 {
    /// Create a new `AffixFuzzer3`.
    #[inline]
    pub fn new() -> Self {
        Self {
            fuzz2001: None,
            fuzz2002: None,
            fuzz2003: None,
            fuzz2004: None,
            fuzz2005: None,
            fuzz2006: None,
            fuzz2007: None,
            fuzz2008: None,
            fuzz2009: None,
            fuzz2010: None,
            fuzz2011: None,
            fuzz2012: None,
            fuzz2013: None,
            fuzz2014: None,
            fuzz2015: None,
            fuzz2016: None,
            fuzz2017: None,
            fuzz2018: None,
        }
    }

    #[inline]
    pub fn with_fuzz2001(
        mut self,
        fuzz2001: impl Into<crate::testing::components::AffixFuzzer1>,
    ) -> Self {
        self.fuzz2001 = Some(fuzz2001.into());
        self
    }

    #[inline]
    pub fn with_fuzz2002(
        mut self,
        fuzz2002: impl Into<crate::testing::components::AffixFuzzer2>,
    ) -> Self {
        self.fuzz2002 = Some(fuzz2002.into());
        self
    }

    #[inline]
    pub fn with_fuzz2003(
        mut self,
        fuzz2003: impl Into<crate::testing::components::AffixFuzzer3>,
    ) -> Self {
        self.fuzz2003 = Some(fuzz2003.into());
        self
    }

    #[inline]
    pub fn with_fuzz2004(
        mut self,
        fuzz2004: impl Into<crate::testing::components::AffixFuzzer4>,
    ) -> Self {
        self.fuzz2004 = Some(fuzz2004.into());
        self
    }

    #[inline]
    pub fn with_fuzz2005(
        mut self,
        fuzz2005: impl Into<crate::testing::components::AffixFuzzer5>,
    ) -> Self {
        self.fuzz2005 = Some(fuzz2005.into());
        self
    }

    #[inline]
    pub fn with_fuzz2006(
        mut self,
        fuzz2006: impl Into<crate::testing::components::AffixFuzzer6>,
    ) -> Self {
        self.fuzz2006 = Some(fuzz2006.into());
        self
    }

    #[inline]
    pub fn with_fuzz2007(
        mut self,
        fuzz2007: impl Into<crate::testing::components::AffixFuzzer7>,
    ) -> Self {
        self.fuzz2007 = Some(fuzz2007.into());
        self
    }

    #[inline]
    pub fn with_fuzz2008(
        mut self,
        fuzz2008: impl Into<crate::testing::components::AffixFuzzer8>,
    ) -> Self {
        self.fuzz2008 = Some(fuzz2008.into());
        self
    }

    #[inline]
    pub fn with_fuzz2009(
        mut self,
        fuzz2009: impl Into<crate::testing::components::AffixFuzzer9>,
    ) -> Self {
        self.fuzz2009 = Some(fuzz2009.into());
        self
    }

    #[inline]
    pub fn with_fuzz2010(
        mut self,
        fuzz2010: impl Into<crate::testing::components::AffixFuzzer10>,
    ) -> Self {
        self.fuzz2010 = Some(fuzz2010.into());
        self
    }

    #[inline]
    pub fn with_fuzz2011(
        mut self,
        fuzz2011: impl Into<crate::testing::components::AffixFuzzer11>,
    ) -> Self {
        self.fuzz2011 = Some(fuzz2011.into());
        self
    }

    #[inline]
    pub fn with_fuzz2012(
        mut self,
        fuzz2012: impl Into<crate::testing::components::AffixFuzzer12>,
    ) -> Self {
        self.fuzz2012 = Some(fuzz2012.into());
        self
    }

    #[inline]
    pub fn with_fuzz2013(
        mut self,
        fuzz2013: impl Into<crate::testing::components::AffixFuzzer13>,
    ) -> Self {
        self.fuzz2013 = Some(fuzz2013.into());
        self
    }

    #[inline]
    pub fn with_fuzz2014(
        mut self,
        fuzz2014: impl Into<crate::testing::components::AffixFuzzer14>,
    ) -> Self {
        self.fuzz2014 = Some(fuzz2014.into());
        self
    }

    #[inline]
    pub fn with_fuzz2015(
        mut self,
        fuzz2015: impl Into<crate::testing::components::AffixFuzzer15>,
    ) -> Self {
        self.fuzz2015 = Some(fuzz2015.into());
        self
    }

    #[inline]
    pub fn with_fuzz2016(
        mut self,
        fuzz2016: impl Into<crate::testing::components::AffixFuzzer16>,
    ) -> Self {
        self.fuzz2016 = Some(fuzz2016.into());
        self
    }

    #[inline]
    pub fn with_fuzz2017(
        mut self,
        fuzz2017: impl Into<crate::testing::components::AffixFuzzer17>,
    ) -> Self {
        self.fuzz2017 = Some(fuzz2017.into());
        self
    }

    #[inline]
    pub fn with_fuzz2018(
        mut self,
        fuzz2018: impl Into<crate::testing::components::AffixFuzzer18>,
    ) -> Self {
        self.fuzz2018 = Some(fuzz2018.into());
        self
    }
}

impl ::re_byte_size::SizeBytes for AffixFuzzer3 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.fuzz2001.heap_size_bytes()
            + self.fuzz2002.heap_size_bytes()
            + self.fuzz2003.heap_size_bytes()
            + self.fuzz2004.heap_size_bytes()
            + self.fuzz2005.heap_size_bytes()
            + self.fuzz2006.heap_size_bytes()
            + self.fuzz2007.heap_size_bytes()
            + self.fuzz2008.heap_size_bytes()
            + self.fuzz2009.heap_size_bytes()
            + self.fuzz2010.heap_size_bytes()
            + self.fuzz2011.heap_size_bytes()
            + self.fuzz2012.heap_size_bytes()
            + self.fuzz2013.heap_size_bytes()
            + self.fuzz2014.heap_size_bytes()
            + self.fuzz2015.heap_size_bytes()
            + self.fuzz2016.heap_size_bytes()
            + self.fuzz2017.heap_size_bytes()
            + self.fuzz2018.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::testing::components::AffixFuzzer1>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer2>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer3>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer4>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer5>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer6>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer7>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer8>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer9>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer10>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer11>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer12>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer13>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer14>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer15>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer16>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer17>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer18>>::is_pod()
    }
}
