// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/keypoint_id.fbs"

#pragma once

#include <cstdint>
#include <memory>
#include <utility>

namespace arrow {
    class DataType;
}

namespace rr {
    namespace components {
        /// A 16-bit ID representing a type of semantic keypoint within a class.
        struct KeypointId {
            uint16_t id;

          public:
            KeypointId(uint16_t id) : id(std::move(id)) {}

            /// Returns the arrow data type this type corresponds to.
            static std::shared_ptr<arrow::DataType> to_arrow_datatype();
        };
    } // namespace components
} // namespace rr
