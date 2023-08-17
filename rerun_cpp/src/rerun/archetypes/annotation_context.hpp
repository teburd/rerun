// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/archetypes/annotation_context.fbs"

#pragma once

#include "../components/annotation_context.hpp"
#include "../data_cell.hpp"
#include "../result.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <utility>

namespace rerun {
    namespace archetypes {
        /// The `AnnotationContext` provides additional information on how to display entities.
        ///
        /// Entities can use `ClassId`s and `KeypointId`s to provide annotations, and
        /// the labels and colors will be looked up in the appropriate
        ///`AnnotationContext`. We use the *first* annotation context we find in the
        /// path-hierarchy when searching up through the ancestors of a given entity
        /// path.
        ///
        /// ## Example
        ///
        ///```
        ///// Log an annotation context to assign a label and color to each class
        ///
        /// #include <rerun.hpp>
        ///
        /// namespace rr = rerun;
        ///
        /// int main() {
        ///    auto rr_stream = rr::RecordingStream("annotation_context_rects");
        ///    rr_stream.connect("127.0.0.1:9876").throw_on_failure();
        ///
        ///    // Log an annotation context to assign a label and color to each class
        ///    rr_stream.log(
        ///        "/",
        ///        rr::archetypes::AnnotationContext({
        ///            rr::datatypes::AnnotationInfo(1, "red", rr::datatypes::Color(255, 0, 0)),
        ///            rr::datatypes::AnnotationInfo(2, "green", rr::datatypes::Color(0, 255, 0)),
        ///        })
        ///    );
        ///
        ///    // Log a batch of 2 arrows with different `class_ids`
        ///    rr_stream.log(
        ///        "arrows",
        ///        rr::archetypes::Arrows3D({{1.0f, 0.0f, 0.0f}, {0.0f, 1.0f,
        ///        0.0f}}).with_class_ids({1, 2})
        ///    );
        /// }
        ///```
        struct AnnotationContext {
            rerun::components::AnnotationContext context;

          public:
            AnnotationContext() = default;

            AnnotationContext(rerun::components::AnnotationContext _context)
                : context(std::move(_context)) {}

            /// Returns the number of primary instances of this archetype.
            size_t num_instances() const {
                return 1;
            }

            /// Creates a list of Rerun DataCell from this archetype.
            Result<std::vector<rerun::DataCell>> to_data_cells() const;
        };
    } // namespace archetypes
} // namespace rerun
