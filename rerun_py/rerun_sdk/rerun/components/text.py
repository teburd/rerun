# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/text.fbs".

# You can extend this class by creating a "TextExt" class in "text_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)

__all__ = ["Text", "TextBatch"]


class Text(datatypes.Utf8, ComponentMixin):
    """**Component**: A string of text, e.g. for labels and text documents."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of TextExt in text_ext.py

    # Note: there are no fields here because Text delegates to datatypes.Utf8


class TextBatch(datatypes.Utf8Batch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.components.Text")


# This is patched in late to avoid circular dependencies.
Text._BATCH_TYPE = TextBatch  # type: ignore[assignment]
