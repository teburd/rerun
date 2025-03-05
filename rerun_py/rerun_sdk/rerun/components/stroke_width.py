# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/stroke_width.fbs".

# You can extend this class by creating a "StrokeWidthExt" class in "stroke_width_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)

__all__ = ["StrokeWidth", "StrokeWidthBatch"]


class StrokeWidth(datatypes.Float32, ComponentMixin):
    """**Component**: The width of a stroke specified in UI points."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of StrokeWidthExt in stroke_width_ext.py

    # Note: there are no fields here because StrokeWidth delegates to datatypes.Float32


class StrokeWidthBatch(datatypes.Float32Batch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.components.StrokeWidth")


# This is patched in late to avoid circular dependencies.
StrokeWidth._BATCH_TYPE = StrokeWidthBatch  # type: ignore[assignment]
