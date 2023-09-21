// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/material.fbs".

#pragma once

#include "../data_cell.hpp"
#include "../datatypes/color.hpp"
#include "../datatypes/material.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <optional>
#include <utility>

namespace arrow {
    class DataType;
    class MemoryPool;
    class StructBuilder;
} // namespace arrow

namespace rerun {
    namespace components {
        struct Material {
            rerun::datatypes::Material material;

            /// Name of the component, used for serialization.
            static const char NAME[];

          public:
            // Extensions to generated type defined in 'material_ext.cpp'

            static Material from_albedo_factor(rerun::datatypes::Color color) {
                return Material(color);
            }

          public:
            Material() = default;

            Material(rerun::datatypes::Material _material) : material(std::move(_material)) {}

            Material& operator=(rerun::datatypes::Material _material) {
                material = std::move(_material);
                return *this;
            }

            Material(std::optional<rerun::datatypes::Color> arg) : material(std::move(arg)) {}

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::StructBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::StructBuilder* builder, const Material* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of Material components.
            static Result<rerun::DataCell> to_data_cell(
                const Material* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
