# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/entity_path.fbs".

# You can extend this class by creating a "EntityPathExt" class in "entity_path_ext.py".

from __future__ import annotations

from collections.abc import Sequence
from typing import TYPE_CHECKING, Any, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseBatch,
)

__all__ = ["EntityPath", "EntityPathArrayLike", "EntityPathBatch", "EntityPathLike"]


@define(init=False)
class EntityPath:
    """**Datatype**: A path to an entity in the `ChunkStore`."""

    def __init__(self: Any, path: EntityPathLike) -> None:
        """Create a new instance of the EntityPath datatype."""

        # You can define your own __init__ function as a member of EntityPathExt in entity_path_ext.py
        self.__attrs_init__(path=path)

    path: str = field(converter=str)

    def __str__(self) -> str:
        return str(self.path)

    def __hash__(self) -> int:
        return hash(self.path)


if TYPE_CHECKING:
    EntityPathLike = Union[EntityPath, str]
else:
    EntityPathLike = Any

EntityPathArrayLike = Union[EntityPath, Sequence[EntityPathLike], Sequence[str]]


class EntityPathBatch(BaseBatch[EntityPathArrayLike]):
    _ARROW_DATATYPE = pa.utf8()

    @staticmethod
    def _native_to_pa_array(data: EntityPathArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, str):
            array: list[str] | npt.ArrayLike = [data]
        elif isinstance(data, Sequence):
            array = [str(datum) for datum in data]
        elif isinstance(data, np.ndarray):
            array = data
        else:
            array = [str(data)]

        return pa.array(array, type=data_type)
