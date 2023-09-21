// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/mat4x4.fbs".

#include "mat4x4.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType> &Mat4x4::arrow_datatype() {
            static const auto datatype =
                arrow::fixed_size_list(arrow::field("item", arrow::float32(), false), 16);
            return datatype;
        }

        Result<std::shared_ptr<arrow::FixedSizeListBuilder>> Mat4x4::new_arrow_array_builder(
            arrow::MemoryPool *memory_pool
        ) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::FixedSizeListBuilder>(
                memory_pool,
                std::make_shared<arrow::FloatBuilder>(memory_pool),
                16
            ));
        }

        Error Mat4x4::fill_arrow_array_builder(
            arrow::FixedSizeListBuilder *builder, const Mat4x4 *elements, size_t num_elements
        ) {
            if (!builder) {
                return Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
            }
            if (!elements) {
                return Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Cannot serialize null pointer to arrow array."
                );
            }

            auto value_builder = static_cast<arrow::FloatBuilder *>(builder->value_builder());

            ARROW_RETURN_NOT_OK(builder->AppendValues(static_cast<int64_t>(num_elements)));
            static_assert(sizeof(elements[0].flat_columns) == sizeof(elements[0]));
            ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                elements[0].flat_columns,
                static_cast<int64_t>(num_elements * 16),
                nullptr
            ));

            return Error::ok();
        }
    } // namespace datatypes
} // namespace rerun
