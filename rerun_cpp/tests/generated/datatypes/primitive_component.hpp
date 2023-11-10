// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy_deps.fbs".

#pragma once

#include <cstdint>
#include <memory>
#include <rerun/result.hpp>

namespace arrow {
    /// \private
    template <typename T>
    class NumericBuilder;

    class DataType;
    class MemoryPool;
    class UInt32Type;
    using UInt32Builder = NumericBuilder<UInt32Type>;
} // namespace arrow

namespace rerun {
    namespace datatypes {
        struct PrimitiveComponent {
            uint32_t value;

          public:
            PrimitiveComponent() = default;

            PrimitiveComponent(uint32_t value_) : value(value_) {}

            PrimitiveComponent& operator=(uint32_t value_) {
                value = value_;
                return *this;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::UInt32Builder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static rerun::Error fill_arrow_array_builder(
                arrow::UInt32Builder* builder, const PrimitiveComponent* elements,
                size_t num_elements
            );
        };
    } // namespace datatypes
} // namespace rerun
