# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/datatypes/mesh_properties.fbs".

# You can extend this class by creating a "MeshPropertiesExt" class in "mesh_properties_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)
from .._converters import (
    to_np_uint32,
)
from .mesh_properties_ext import MeshPropertiesExt

__all__ = [
    "MeshProperties",
    "MeshPropertiesArray",
    "MeshPropertiesArrayLike",
    "MeshPropertiesLike",
    "MeshPropertiesType",
]


@define
class MeshProperties(MeshPropertiesExt):
    # You can define your own __init__ function as a member of MeshPropertiesExt in mesh_properties_ext.py

    vertex_indices: npt.NDArray[np.uint32] | None = field(default=None, converter=to_np_uint32)
    """
    If specified, is a flattened array of indices that describe the mesh's triangles,
    i.e. its length must be divisible by 3.
    """

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of MeshPropertiesExt in mesh_properties_ext.py
        return np.asarray(self.vertex_indices, dtype=dtype)


MeshPropertiesLike = MeshProperties
MeshPropertiesArrayLike = Union[
    MeshProperties,
    Sequence[MeshPropertiesLike],
]


# --- Arrow support ---


class MeshPropertiesType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct(
                [
                    pa.field(
                        "vertex_indices",
                        pa.list_(pa.field("item", pa.uint32(), nullable=False, metadata={})),
                        nullable=True,
                        metadata={},
                    )
                ]
            ),
            "rerun.datatypes.MeshProperties",
        )


class MeshPropertiesArray(BaseExtensionArray[MeshPropertiesArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.MeshProperties"
    _EXTENSION_TYPE = MeshPropertiesType

    @staticmethod
    def _native_to_pa_array(data: MeshPropertiesArrayLike, data_type: pa.DataType) -> pa.Array:
        return MeshPropertiesExt.native_to_pa_array_override(data, data_type)


MeshPropertiesType._ARRAY_TYPE = MeshPropertiesArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(MeshPropertiesType())
