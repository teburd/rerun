# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/force_center.fbs".

# You can extend this class by creating a "ForceCenterExt" class in "force_center_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ... import datatypes
from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["ForceCenter"]


@define(str=False, repr=False, init=False)
class ForceCenter(Archetype):
    """**Archetype**: Tries to move the center of mass of the graph to the origin."""

    def __init__(
        self: Any, *, enabled: datatypes.BoolLike | None = None, strength: datatypes.Float64Like | None = None
    ) -> None:
        """
        Create a new instance of the ForceCenter archetype.

        Parameters
        ----------
        enabled:
            Whether the center force is enabled.

            The center force tries to move the center of mass of the graph towards the origin.
        strength:
            The strength of the force.

        """

        # You can define your own __init__ function as a member of ForceCenterExt in force_center_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(enabled=enabled, strength=strength)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            enabled=None,
            strength=None,
        )

    @classmethod
    def _clear(cls) -> ForceCenter:
        """Produce an empty ForceCenter, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    @classmethod
    def from_fields(
        cls,
        *,
        clear_unset: bool = False,
        enabled: datatypes.BoolLike | None = None,
        strength: datatypes.Float64Like | None = None,
    ) -> ForceCenter:
        """
        Update only some specific fields of a `ForceCenter`.

        Parameters
        ----------
        clear_unset:
            If true, all unspecified fields will be explicitly cleared.
        enabled:
            Whether the center force is enabled.

            The center force tries to move the center of mass of the graph towards the origin.
        strength:
            The strength of the force.

        """

        inst = cls.__new__(cls)
        with catch_and_log_exceptions(context=cls.__name__):
            kwargs = {
                "enabled": enabled,
                "strength": strength,
            }

            if clear_unset:
                kwargs = {k: v if v is not None else [] for k, v in kwargs.items()}  # type: ignore[misc]

            inst.__attrs_init__(**kwargs)
            return inst

        inst.__attrs_clear__()
        return inst

    @classmethod
    def cleared(cls) -> ForceCenter:
        """Clear all the fields of a `ForceCenter`."""
        return cls.from_fields(clear_unset=True)

    enabled: blueprint_components.EnabledBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=blueprint_components.EnabledBatch._converter,  # type: ignore[misc]
    )
    # Whether the center force is enabled.
    #
    # The center force tries to move the center of mass of the graph towards the origin.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    strength: blueprint_components.ForceStrengthBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=blueprint_components.ForceStrengthBatch._converter,  # type: ignore[misc]
    )
    # The strength of the force.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
