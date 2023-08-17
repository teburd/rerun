// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/instance_key.fbs"

#include "instance_key.hpp"

#include "../arrow.hpp"

#include <arrow/api.h>

namespace rerun {
    namespace components {
        const char* InstanceKey::NAME = "rerun.instance_key";

        const std::shared_ptr<arrow::DataType>& InstanceKey::to_arrow_datatype() {
            static const auto datatype = arrow::uint64();
            return datatype;
        }

        arrow::Result<std::shared_ptr<arrow::UInt64Builder>> InstanceKey::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (!memory_pool) {
                return arrow::Status::Invalid("Memory pool is null.");
            }

            return arrow::Result(std::make_shared<arrow::UInt64Builder>(memory_pool));
        }

        arrow::Status InstanceKey::fill_arrow_array_builder(
            arrow::UInt64Builder* builder, const InstanceKey* elements, size_t num_elements
        ) {
            if (!builder) {
                return arrow::Status::Invalid("Passed array builder is null.");
            }
            if (!elements) {
                return arrow::Status::Invalid("Cannot serialize null pointer to arrow array.");
            }

            static_assert(sizeof(*elements) == sizeof(elements->value));
            ARROW_RETURN_NOT_OK(
                builder->AppendValues(&elements->value, static_cast<int64_t>(num_elements))
            );

            return arrow::Status::OK();
        }

        Result<rerun::DataCell> InstanceKey::to_data_cell(
            const InstanceKey* instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool* pool = arrow::default_memory_pool();

            ARROW_ASSIGN_OR_RAISE(auto builder, InstanceKey::new_arrow_array_builder(pool));
            if (instances && num_instances > 0) {
                ARROW_RETURN_NOT_OK(
                    InstanceKey::fill_arrow_array_builder(builder.get(), instances, num_instances)
                );
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema = arrow::schema(
                {arrow::field(InstanceKey::NAME, InstanceKey::to_arrow_datatype(), false)}
            );

            rerun::DataCell cell;
            cell.component_name = InstanceKey::NAME;
            const auto result = rerun::ipc_from_table(*arrow::Table::Make(schema, {array}));
            if (result.is_err()) {
                return result.error;
            }
            cell.buffer = std::move(result.value);

            return cell;
        }
    } // namespace components
} // namespace rerun
