# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from .angle import Angle, AngleArrayLike, AngleBatch, AngleLike, AngleType
    from .annotation_info import (
        AnnotationInfo,
        AnnotationInfoArrayLike,
        AnnotationInfoBatch,
        AnnotationInfoLike,
        AnnotationInfoType,
    )
    from .class_description import (
        ClassDescription,
        ClassDescriptionArrayLike,
        ClassDescriptionBatch,
        ClassDescriptionLike,
        ClassDescriptionType,
    )
    from .class_description_map_elem import (
        ClassDescriptionMapElem,
        ClassDescriptionMapElemArrayLike,
        ClassDescriptionMapElemBatch,
        ClassDescriptionMapElemLike,
        ClassDescriptionMapElemType,
    )
    from .class_id import ClassId, ClassIdArrayLike, ClassIdBatch, ClassIdLike, ClassIdType
    from .float32 import Float32, Float32ArrayLike, Float32Batch, Float32Like, Float32Type
    from .keypoint_id import KeypointId, KeypointIdArrayLike, KeypointIdBatch, KeypointIdLike, KeypointIdType
    from .keypoint_pair import (
        KeypointPair,
        KeypointPairArrayLike,
        KeypointPairBatch,
        KeypointPairLike,
        KeypointPairType,
    )
    from .mat3x3 import Mat3x3, Mat3x3ArrayLike, Mat3x3Batch, Mat3x3Like, Mat3x3Type
    from .mat4x4 import Mat4x4, Mat4x4ArrayLike, Mat4x4Batch, Mat4x4Like, Mat4x4Type
    from .material import Material, MaterialArrayLike, MaterialBatch, MaterialLike, MaterialType
    from .mesh_properties import (
        MeshProperties,
        MeshPropertiesArrayLike,
        MeshPropertiesBatch,
        MeshPropertiesLike,
        MeshPropertiesType,
    )
    from .quaternion import Quaternion, QuaternionArrayLike, QuaternionBatch, QuaternionLike, QuaternionType
    from .rgba32 import Rgba32, Rgba32ArrayLike, Rgba32Batch, Rgba32Like, Rgba32Type
    from .rotation3d import Rotation3D, Rotation3DArrayLike, Rotation3DBatch, Rotation3DLike, Rotation3DType
    from .rotation_axis_angle import (
        RotationAxisAngle,
        RotationAxisAngleArrayLike,
        RotationAxisAngleBatch,
        RotationAxisAngleLike,
        RotationAxisAngleType,
    )
    from .scale3d import Scale3D, Scale3DArrayLike, Scale3DBatch, Scale3DLike, Scale3DType
    from .tensor_buffer import (
        TensorBuffer,
        TensorBufferArrayLike,
        TensorBufferBatch,
        TensorBufferLike,
        TensorBufferType,
    )
    from .tensor_data import TensorData, TensorDataArrayLike, TensorDataBatch, TensorDataLike, TensorDataType
    from .tensor_dimension import (
        TensorDimension,
        TensorDimensionArrayLike,
        TensorDimensionBatch,
        TensorDimensionLike,
        TensorDimensionType,
    )
    from .transform3d import Transform3D, Transform3DArrayLike, Transform3DBatch, Transform3DLike, Transform3DType
    from .translation_and_mat3x3 import (
        TranslationAndMat3x3,
        TranslationAndMat3x3ArrayLike,
        TranslationAndMat3x3Batch,
        TranslationAndMat3x3Like,
        TranslationAndMat3x3Type,
    )
    from .translation_rotation_scale3d import (
        TranslationRotationScale3D,
        TranslationRotationScale3DArrayLike,
        TranslationRotationScale3DBatch,
        TranslationRotationScale3DLike,
        TranslationRotationScale3DType,
    )
    from .uint32 import UInt32, UInt32ArrayLike, UInt32Batch, UInt32Like, UInt32Type
    from .utf8 import Utf8, Utf8ArrayLike, Utf8Batch, Utf8Like, Utf8Type
    from .uvec2d import UVec2D, UVec2DArrayLike, UVec2DBatch, UVec2DLike, UVec2DType
    from .uvec3d import UVec3D, UVec3DArrayLike, UVec3DBatch, UVec3DLike, UVec3DType
    from .uvec4d import UVec4D, UVec4DArrayLike, UVec4DBatch, UVec4DLike, UVec4DType
    from .vec2d import Vec2D, Vec2DArrayLike, Vec2DBatch, Vec2DLike, Vec2DType
    from .vec3d import Vec3D, Vec3DArrayLike, Vec3DBatch, Vec3DLike, Vec3DType
    from .vec4d import Vec4D, Vec4DArrayLike, Vec4DBatch, Vec4DLike, Vec4DType

__all__ = [
    "Angle",
    "AngleArrayLike",
    "AngleBatch",
    "AngleLike",
    "AngleType",
    "AnnotationInfo",
    "AnnotationInfoArrayLike",
    "AnnotationInfoBatch",
    "AnnotationInfoLike",
    "AnnotationInfoType",
    "ClassDescription",
    "ClassDescriptionArrayLike",
    "ClassDescriptionBatch",
    "ClassDescriptionLike",
    "ClassDescriptionMapElem",
    "ClassDescriptionMapElemArrayLike",
    "ClassDescriptionMapElemBatch",
    "ClassDescriptionMapElemLike",
    "ClassDescriptionMapElemType",
    "ClassDescriptionType",
    "ClassId",
    "ClassIdArrayLike",
    "ClassIdBatch",
    "ClassIdLike",
    "ClassIdType",
    "Float32",
    "Float32ArrayLike",
    "Float32Batch",
    "Float32Like",
    "Float32Type",
    "KeypointId",
    "KeypointIdArrayLike",
    "KeypointIdBatch",
    "KeypointIdLike",
    "KeypointIdType",
    "KeypointPair",
    "KeypointPairArrayLike",
    "KeypointPairBatch",
    "KeypointPairLike",
    "KeypointPairType",
    "Mat3x3",
    "Mat3x3ArrayLike",
    "Mat3x3Batch",
    "Mat3x3Like",
    "Mat3x3Type",
    "Mat4x4",
    "Mat4x4ArrayLike",
    "Mat4x4Batch",
    "Mat4x4Like",
    "Mat4x4Type",
    "Material",
    "MaterialArrayLike",
    "MaterialBatch",
    "MaterialLike",
    "MaterialType",
    "MeshProperties",
    "MeshPropertiesArrayLike",
    "MeshPropertiesBatch",
    "MeshPropertiesLike",
    "MeshPropertiesType",
    "Quaternion",
    "QuaternionArrayLike",
    "QuaternionBatch",
    "QuaternionLike",
    "QuaternionType",
    "Rgba32",
    "Rgba32ArrayLike",
    "Rgba32Batch",
    "Rgba32Like",
    "Rgba32Type",
    "Rotation3D",
    "Rotation3DArrayLike",
    "Rotation3DBatch",
    "Rotation3DLike",
    "Rotation3DType",
    "RotationAxisAngle",
    "RotationAxisAngleArrayLike",
    "RotationAxisAngleBatch",
    "RotationAxisAngleLike",
    "RotationAxisAngleType",
    "Scale3D",
    "Scale3DArrayLike",
    "Scale3DBatch",
    "Scale3DLike",
    "Scale3DType",
    "TensorBuffer",
    "TensorBufferArrayLike",
    "TensorBufferBatch",
    "TensorBufferLike",
    "TensorBufferType",
    "TensorData",
    "TensorDataArrayLike",
    "TensorDataBatch",
    "TensorDataLike",
    "TensorDataType",
    "TensorDimension",
    "TensorDimensionArrayLike",
    "TensorDimensionBatch",
    "TensorDimensionLike",
    "TensorDimensionType",
    "Transform3D",
    "Transform3DArrayLike",
    "Transform3DBatch",
    "Transform3DLike",
    "Transform3DType",
    "TranslationAndMat3x3",
    "TranslationAndMat3x3ArrayLike",
    "TranslationAndMat3x3Batch",
    "TranslationAndMat3x3Like",
    "TranslationAndMat3x3Type",
    "TranslationRotationScale3D",
    "TranslationRotationScale3DArrayLike",
    "TranslationRotationScale3DBatch",
    "TranslationRotationScale3DLike",
    "TranslationRotationScale3DType",
    "UInt32",
    "UInt32ArrayLike",
    "UInt32Batch",
    "UInt32Like",
    "UInt32Type",
    "UVec2D",
    "UVec2DArrayLike",
    "UVec2DBatch",
    "UVec2DLike",
    "UVec2DType",
    "UVec3D",
    "UVec3DArrayLike",
    "UVec3DBatch",
    "UVec3DLike",
    "UVec3DType",
    "UVec4D",
    "UVec4DArrayLike",
    "UVec4DBatch",
    "UVec4DLike",
    "UVec4DType",
    "Utf8",
    "Utf8ArrayLike",
    "Utf8Batch",
    "Utf8Like",
    "Utf8Type",
    "Vec2D",
    "Vec2DArrayLike",
    "Vec2DBatch",
    "Vec2DLike",
    "Vec2DType",
    "Vec3D",
    "Vec3DArrayLike",
    "Vec3DBatch",
    "Vec3DLike",
    "Vec3DType",
    "Vec4D",
    "Vec4DArrayLike",
    "Vec4DBatch",
    "Vec4DLike",
    "Vec4DType",
]

module_content: dict[str, str] = {
    "Angle": "angle",
    "AngleArrayLike": "angle",
    "AngleBatch": "angle",
    "AngleLike": "angle",
    "AngleType": "angle",
    "AnnotationInfo": "annotation_info",
    "AnnotationInfoArrayLike": "annotation_info",
    "AnnotationInfoBatch": "annotation_info",
    "AnnotationInfoLike": "annotation_info",
    "AnnotationInfoType": "annotation_info",
    "ClassDescription": "class_description",
    "ClassDescriptionArrayLike": "class_description",
    "ClassDescriptionBatch": "class_description",
    "ClassDescriptionLike": "class_description",
    "ClassDescriptionType": "class_description",
    "ClassDescriptionMapElem": "class_description_map_elem",
    "ClassDescriptionMapElemArrayLike": "class_description_map_elem",
    "ClassDescriptionMapElemBatch": "class_description_map_elem",
    "ClassDescriptionMapElemLike": "class_description_map_elem",
    "ClassDescriptionMapElemType": "class_description_map_elem",
    "ClassId": "class_id",
    "ClassIdArrayLike": "class_id",
    "ClassIdBatch": "class_id",
    "ClassIdLike": "class_id",
    "ClassIdType": "class_id",
    "Float32": "float32",
    "Float32ArrayLike": "float32",
    "Float32Batch": "float32",
    "Float32Like": "float32",
    "Float32Type": "float32",
    "KeypointId": "keypoint_id",
    "KeypointIdArrayLike": "keypoint_id",
    "KeypointIdBatch": "keypoint_id",
    "KeypointIdLike": "keypoint_id",
    "KeypointIdType": "keypoint_id",
    "KeypointPair": "keypoint_pair",
    "KeypointPairArrayLike": "keypoint_pair",
    "KeypointPairBatch": "keypoint_pair",
    "KeypointPairLike": "keypoint_pair",
    "KeypointPairType": "keypoint_pair",
    "Mat3x3": "mat3x3",
    "Mat3x3ArrayLike": "mat3x3",
    "Mat3x3Batch": "mat3x3",
    "Mat3x3Like": "mat3x3",
    "Mat3x3Type": "mat3x3",
    "Mat4x4": "mat4x4",
    "Mat4x4ArrayLike": "mat4x4",
    "Mat4x4Batch": "mat4x4",
    "Mat4x4Like": "mat4x4",
    "Mat4x4Type": "mat4x4",
    "Material": "material",
    "MaterialArrayLike": "material",
    "MaterialBatch": "material",
    "MaterialLike": "material",
    "MaterialType": "material",
    "MeshProperties": "mesh_properties",
    "MeshPropertiesArrayLike": "mesh_properties",
    "MeshPropertiesBatch": "mesh_properties",
    "MeshPropertiesLike": "mesh_properties",
    "MeshPropertiesType": "mesh_properties",
    "Quaternion": "quaternion",
    "QuaternionArrayLike": "quaternion",
    "QuaternionBatch": "quaternion",
    "QuaternionLike": "quaternion",
    "QuaternionType": "quaternion",
    "Rgba32": "rgba32",
    "Rgba32ArrayLike": "rgba32",
    "Rgba32Batch": "rgba32",
    "Rgba32Like": "rgba32",
    "Rgba32Type": "rgba32",
    "Rotation3D": "rotation3d",
    "Rotation3DArrayLike": "rotation3d",
    "Rotation3DBatch": "rotation3d",
    "Rotation3DLike": "rotation3d",
    "Rotation3DType": "rotation3d",
    "RotationAxisAngle": "rotation_axis_angle",
    "RotationAxisAngleArrayLike": "rotation_axis_angle",
    "RotationAxisAngleBatch": "rotation_axis_angle",
    "RotationAxisAngleLike": "rotation_axis_angle",
    "RotationAxisAngleType": "rotation_axis_angle",
    "Scale3D": "scale3d",
    "Scale3DArrayLike": "scale3d",
    "Scale3DBatch": "scale3d",
    "Scale3DLike": "scale3d",
    "Scale3DType": "scale3d",
    "TensorBuffer": "tensor_buffer",
    "TensorBufferArrayLike": "tensor_buffer",
    "TensorBufferBatch": "tensor_buffer",
    "TensorBufferLike": "tensor_buffer",
    "TensorBufferType": "tensor_buffer",
    "TensorData": "tensor_data",
    "TensorDataArrayLike": "tensor_data",
    "TensorDataBatch": "tensor_data",
    "TensorDataLike": "tensor_data",
    "TensorDataType": "tensor_data",
    "TensorDimension": "tensor_dimension",
    "TensorDimensionArrayLike": "tensor_dimension",
    "TensorDimensionBatch": "tensor_dimension",
    "TensorDimensionLike": "tensor_dimension",
    "TensorDimensionType": "tensor_dimension",
    "Transform3D": "transform3d",
    "Transform3DArrayLike": "transform3d",
    "Transform3DBatch": "transform3d",
    "Transform3DLike": "transform3d",
    "Transform3DType": "transform3d",
    "TranslationAndMat3x3": "translation_and_mat3x3",
    "TranslationAndMat3x3ArrayLike": "translation_and_mat3x3",
    "TranslationAndMat3x3Batch": "translation_and_mat3x3",
    "TranslationAndMat3x3Like": "translation_and_mat3x3",
    "TranslationAndMat3x3Type": "translation_and_mat3x3",
    "TranslationRotationScale3D": "translation_rotation_scale3d",
    "TranslationRotationScale3DArrayLike": "translation_rotation_scale3d",
    "TranslationRotationScale3DBatch": "translation_rotation_scale3d",
    "TranslationRotationScale3DLike": "translation_rotation_scale3d",
    "TranslationRotationScale3DType": "translation_rotation_scale3d",
    "UInt32": "uint32",
    "UInt32ArrayLike": "uint32",
    "UInt32Batch": "uint32",
    "UInt32Like": "uint32",
    "UInt32Type": "uint32",
    "Utf8": "utf8",
    "Utf8ArrayLike": "utf8",
    "Utf8Batch": "utf8",
    "Utf8Like": "utf8",
    "Utf8Type": "utf8",
    "UVec2D": "uvec2d",
    "UVec2DArrayLike": "uvec2d",
    "UVec2DBatch": "uvec2d",
    "UVec2DLike": "uvec2d",
    "UVec2DType": "uvec2d",
    "UVec3D": "uvec3d",
    "UVec3DArrayLike": "uvec3d",
    "UVec3DBatch": "uvec3d",
    "UVec3DLike": "uvec3d",
    "UVec3DType": "uvec3d",
    "UVec4D": "uvec4d",
    "UVec4DArrayLike": "uvec4d",
    "UVec4DBatch": "uvec4d",
    "UVec4DLike": "uvec4d",
    "UVec4DType": "uvec4d",
    "Vec2D": "vec2d",
    "Vec2DArrayLike": "vec2d",
    "Vec2DBatch": "vec2d",
    "Vec2DLike": "vec2d",
    "Vec2DType": "vec2d",
    "Vec3D": "vec3d",
    "Vec3DArrayLike": "vec3d",
    "Vec3DBatch": "vec3d",
    "Vec3DLike": "vec3d",
    "Vec3DType": "vec3d",
    "Vec4D": "vec4d",
    "Vec4DArrayLike": "vec4d",
    "Vec4DBatch": "vec4d",
    "Vec4DLike": "vec4d",
    "Vec4DType": "vec4d",
}


def __getattr__(name: str) -> Any:
    from importlib import import_module

    if name in module_content:
        module = import_module(f".{module_content[name]}", __name__)
        return getattr(module, name)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")
