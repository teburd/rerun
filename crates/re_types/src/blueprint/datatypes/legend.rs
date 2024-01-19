// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/datatypes/legend.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: Configuration for the legend of a plot.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Legend {
    /// Whether or not the legend should be displayed.
    pub visible: bool,

    /// Where should the legend be located.
    ///
    /// Allowed values:
    ///  - LeftTop = 1,
    ///  - RightTop = 2,
    ///  - LeftBottom = 3,
    ///  - RightBottom = 4
    pub location: Option<u8>,
}

impl ::re_types_core::SizeBytes for Legend {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.visible.heap_size_bytes() + self.location.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <bool>::is_pod() && <Option<u8>>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(Legend);

impl ::re_types_core::Loggable for Legend {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.blueprint.datatypes.Legend".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "visible".to_owned(),
                data_type: DataType::Boolean,
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "location".to_owned(),
                data_type: DataType::UInt8,
                is_nullable: true,
                metadata: [].into(),
            },
        ])
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::blueprint::datatypes::Legend>::arrow_datatype(),
                vec![
                    {
                        let (somes, visible): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { visible, .. } = &**datum;
                                    visible.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let visible_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        BooleanArray::new(
                            DataType::Boolean,
                            visible.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            visible_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, location): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| {
                                        let Self { location, .. } = &**datum;
                                        location.clone()
                                    })
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let location_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt8,
                            location
                                .into_iter()
                                .map(|v| v.unwrap_or_default())
                                .collect(),
                            location_bitmap,
                        )
                        .boxed()
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![
                            Field {
                                name: "visible".to_owned(),
                                data_type: DataType::Boolean,
                                is_nullable: false,
                                metadata: [].into(),
                            },
                            Field {
                                name: "location".to_owned(),
                                data_type: DataType::UInt8,
                                is_nullable: true,
                                metadata: [].into(),
                            },
                        ]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.blueprint.datatypes.Legend")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let visible = {
                    if !arrays_by_name.contains_key("visible") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "visible",
                        ))
                        .with_context("rerun.blueprint.datatypes.Legend");
                    }
                    let arrow_data = &**arrays_by_name["visible"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<BooleanArray>()
                        .ok_or_else(|| {
                            DeserializationError::datatype_mismatch(
                                DataType::Boolean,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.blueprint.datatypes.Legend#visible")?
                        .into_iter()
                };
                let location = {
                    if !arrays_by_name.contains_key("location") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "location",
                        ))
                        .with_context("rerun.blueprint.datatypes.Legend");
                    }
                    let arrow_data = &**arrays_by_name["location"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt8Array>()
                        .ok_or_else(|| {
                            DeserializationError::datatype_mismatch(
                                DataType::UInt8,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.blueprint.datatypes.Legend#location")?
                        .into_iter()
                        .map(|opt| opt.copied())
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(visible, location),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(visible, location)| {
                        Ok(Self {
                            visible: visible
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.blueprint.datatypes.Legend#visible")?,
                            location,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.blueprint.datatypes.Legend")?
            }
        })
    }
}
