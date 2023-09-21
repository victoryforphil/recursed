// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/mesh_properties.fbs".

#include "mesh_properties.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType> &MeshProperties::arrow_datatype() {
            static const auto datatype = arrow::struct_({
                arrow::field(
                    "vertex_indices",
                    arrow::list(arrow::field("item", arrow::uint32(), false)),
                    true
                ),
            });
            return datatype;
        }

        Result<std::shared_ptr<arrow::StructBuilder>> MeshProperties::new_arrow_array_builder(
            arrow::MemoryPool *memory_pool
        ) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::StructBuilder>(
                arrow_datatype(),
                memory_pool,
                std::vector<std::shared_ptr<arrow::ArrayBuilder>>({
                    std::make_shared<arrow::ListBuilder>(
                        memory_pool,
                        std::make_shared<arrow::UInt32Builder>(memory_pool)
                    ),
                })
            ));
        }

        Error MeshProperties::fill_arrow_array_builder(
            arrow::StructBuilder *builder, const MeshProperties *elements, size_t num_elements
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

            {
                auto field_builder = static_cast<arrow::ListBuilder *>(builder->field_builder(0));
                auto value_builder =
                    static_cast<arrow::UInt32Builder *>(field_builder->value_builder());
                ARROW_RETURN_NOT_OK(field_builder->Reserve(static_cast<int64_t>(num_elements)));
                ARROW_RETURN_NOT_OK(value_builder->Reserve(static_cast<int64_t>(num_elements * 1)));

                for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                    const auto &element = elements[elem_idx];
                    if (element.vertex_indices.has_value()) {
                        ARROW_RETURN_NOT_OK(field_builder->Append());
                        ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                            element.vertex_indices.value().data(),
                            static_cast<int64_t>(element.vertex_indices.value().size()),
                            nullptr
                        ));
                    } else {
                        ARROW_RETURN_NOT_OK(field_builder->AppendNull());
                    }
                }
            }
            ARROW_RETURN_NOT_OK(builder->AppendValues(static_cast<int64_t>(num_elements), nullptr));

            return Error::ok();
        }
    } // namespace datatypes
} // namespace rerun
