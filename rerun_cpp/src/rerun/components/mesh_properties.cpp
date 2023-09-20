// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/mesh_properties.fbs".

#include "mesh_properties.hpp"

#include "../arrow.hpp"
#include "../datatypes/mesh_properties.hpp"

#include <arrow/builder.h>
#include <arrow/table.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace components {
        const char MeshProperties::NAME[] = "rerun.components.MeshProperties";

        const std::shared_ptr<arrow::DataType> &MeshProperties::arrow_datatype() {
            static const auto datatype = rerun::datatypes::MeshProperties::arrow_datatype();
            return datatype;
        }

        Result<std::shared_ptr<arrow::StructBuilder>> MeshProperties::new_arrow_array_builder(
            arrow::MemoryPool *memory_pool
        ) {
            if (!memory_pool) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(
                rerun::datatypes::MeshProperties::new_arrow_array_builder(memory_pool).value
            );
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

            static_assert(sizeof(rerun::datatypes::MeshProperties) == sizeof(MeshProperties));
            RR_RETURN_NOT_OK(rerun::datatypes::MeshProperties::fill_arrow_array_builder(
                builder,
                reinterpret_cast<const rerun::datatypes::MeshProperties *>(elements),
                num_elements
            ));

            return Error::ok();
        }

        Result<rerun::DataCell> MeshProperties::to_data_cell(
            const MeshProperties *instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool *pool = arrow::default_memory_pool();

            auto builder_result = MeshProperties::new_arrow_array_builder(pool);
            RR_RETURN_NOT_OK(builder_result.error);
            auto builder = std::move(builder_result.value);
            if (instances && num_instances > 0) {
                RR_RETURN_NOT_OK(MeshProperties::fill_arrow_array_builder(
                    builder.get(),
                    instances,
                    num_instances
                ));
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema = arrow::schema(
                {arrow::field(MeshProperties::NAME, MeshProperties::arrow_datatype(), false)}
            );

            rerun::DataCell cell;
            cell.component_name = MeshProperties::NAME;
            const auto ipc_result = rerun::ipc_from_table(*arrow::Table::Make(schema, {array}));
            RR_RETURN_NOT_OK(ipc_result.error);
            cell.buffer = std::move(ipc_result.value);

            return cell;
        }
    } // namespace components
} // namespace rerun
