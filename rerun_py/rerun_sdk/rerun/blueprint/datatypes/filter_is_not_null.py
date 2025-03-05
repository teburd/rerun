# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/datatypes/filter_is_not_null.fbs".

# You can extend this class by creating a "FilterIsNotNullExt" class in "filter_is_not_null_ext.py".

from __future__ import annotations

from collections.abc import Sequence
from typing import TYPE_CHECKING, Any, Union

import pyarrow as pa
from attrs import define, field

from ... import datatypes
from ..._baseclasses import (
    BaseBatch,
)
from ...blueprint import datatypes as blueprint_datatypes
from .filter_is_not_null_ext import FilterIsNotNullExt

__all__ = ["FilterIsNotNull", "FilterIsNotNullArrayLike", "FilterIsNotNullBatch", "FilterIsNotNullLike"]


def _filter_is_not_null__active__special_field_converter_override(x: datatypes.BoolLike) -> datatypes.Bool:
    if isinstance(x, datatypes.Bool):
        return x
    else:
        return datatypes.Bool(x)


@define(init=False)
class FilterIsNotNull(FilterIsNotNullExt):
    """**Datatype**: Configuration for the filter is not null feature of the dataframe view."""

    def __init__(
        self: Any, active: datatypes.BoolLike, column: blueprint_datatypes.ComponentColumnSelectorLike
    ) -> None:
        """
        Create a new instance of the FilterIsNotNull datatype.

        Parameters
        ----------
        active:
            Whether the filter by event feature is active.
        column:
            The column used when the filter by event feature is used.

        """

        # You can define your own __init__ function as a member of FilterIsNotNullExt in filter_is_not_null_ext.py
        self.__attrs_init__(active=active, column=column)

    active: datatypes.Bool = field(converter=_filter_is_not_null__active__special_field_converter_override)
    # Whether the filter by event feature is active.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    column: blueprint_datatypes.ComponentColumnSelector = field()
    # The column used when the filter by event feature is used.
    #
    # (Docstring intentionally commented out to hide this field from the docs)


if TYPE_CHECKING:
    FilterIsNotNullLike = Union[FilterIsNotNull, blueprint_datatypes.ComponentColumnSelectorLike]
else:
    FilterIsNotNullLike = Any

FilterIsNotNullArrayLike = Union[
    FilterIsNotNull,
    Sequence[FilterIsNotNullLike],
]


class FilterIsNotNullBatch(BaseBatch[FilterIsNotNullArrayLike]):
    _ARROW_DATATYPE = pa.struct([
        pa.field("active", pa.bool_(), nullable=False, metadata={}),
        pa.field(
            "column",
            pa.struct([
                pa.field("entity_path", pa.utf8(), nullable=False, metadata={}),
                pa.field("component", pa.utf8(), nullable=False, metadata={}),
            ]),
            nullable=False,
            metadata={},
        ),
    ])

    @staticmethod
    def _native_to_pa_array(data: FilterIsNotNullArrayLike, data_type: pa.DataType) -> pa.Array:
        return FilterIsNotNullExt.native_to_pa_array_override(data, data_type)
