# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/map_background.fbs".

# You can extend this class by creating a "MapBackgroundExt" class in "map_background_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["MapBackground"]


@define(str=False, repr=False, init=False)
class MapBackground(Archetype):
    """**Archetype**: Configuration for the background map of the map view."""

    def __init__(self: Any, provider: blueprint_components.MapProviderLike) -> None:
        """
        Create a new instance of the MapBackground archetype.

        Parameters
        ----------
        provider:
            Map provider and style to use.

            **Note**: Requires a Mapbox API key in the `RERUN_MAPBOX_ACCESS_TOKEN` environment variable.

        """

        # You can define your own __init__ function as a member of MapBackgroundExt in map_background_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(provider=provider)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            provider=None,
        )

    @classmethod
    def _clear(cls) -> MapBackground:
        """Produce an empty MapBackground, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    @classmethod
    def from_fields(
        cls,
        *,
        clear_unset: bool = False,
        provider: blueprint_components.MapProviderLike | None = None,
    ) -> MapBackground:
        """
        Update only some specific fields of a `MapBackground`.

        Parameters
        ----------
        clear_unset:
            If true, all unspecified fields will be explicitly cleared.
        provider:
            Map provider and style to use.

            **Note**: Requires a Mapbox API key in the `RERUN_MAPBOX_ACCESS_TOKEN` environment variable.

        """

        inst = cls.__new__(cls)
        with catch_and_log_exceptions(context=cls.__name__):
            kwargs = {
                "provider": provider,
            }

            if clear_unset:
                kwargs = {k: v if v is not None else [] for k, v in kwargs.items()}  # type: ignore[misc]

            inst.__attrs_init__(**kwargs)
            return inst

        inst.__attrs_clear__()
        return inst

    @classmethod
    def cleared(cls) -> MapBackground:
        """Clear all the fields of a `MapBackground`."""
        return cls.from_fields(clear_unset=True)

    provider: blueprint_components.MapProviderBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=blueprint_components.MapProviderBatch._converter,  # type: ignore[misc]
    )
    # Map provider and style to use.
    #
    # **Note**: Requires a Mapbox API key in the `RERUN_MAPBOX_ACCESS_TOKEN` environment variable.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
