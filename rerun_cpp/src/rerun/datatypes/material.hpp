// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/material.fbs".

#pragma once

#include "../result.hpp"
#include "color.hpp"

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
    namespace datatypes {
        struct Material {
            /// Optional color multiplier.
            std::optional<rerun::datatypes::Color> albedo_factor;

          public:
            Material() = default;

            Material(std::optional<rerun::datatypes::Color> _albedo_factor)
                : albedo_factor(std::move(_albedo_factor)) {}

            Material& operator=(std::optional<rerun::datatypes::Color> _albedo_factor) {
                albedo_factor = std::move(_albedo_factor);
                return *this;
            }

            Material(uint32_t arg) : albedo_factor(std::move(arg)) {}

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
        };
    } // namespace datatypes
} // namespace rerun
