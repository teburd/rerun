// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/video.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/blob.hpp"
#include "../components/media_type.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <filesystem>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A video file.
    ///
    /// NOTE: Videos can only be viewed in the Rerun web viewer.
    /// Only MP4 and AV1 is currently supported, and not in all browsers.
    /// Follow <https://github.com/rerun-io/rerun/issues/7298> for updates on the native support.
    ///
    /// ⚠ **This is an experimental API! It is not fully supported, and is likely to change significantly in future versions.**
    struct AssetVideo {
        /// The asset's bytes.
        rerun::components::Blob blob;

        /// The Media Type of the asset.
        ///
        /// Supported values:
        /// * `video/mp4`
        ///
        /// If omitted, the viewer will try to guess from the data blob.
        /// If it cannot guess, it won't be able to render the asset.
        std::optional<rerun::components::MediaType> media_type;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.AssetVideoIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public: // START of extensions from asset_video_ext.cpp:
        /// Creates a new `AssetVideo` from the file contents at `path`.
        ///
        /// The `MediaType` will be guessed from the file extension.
        ///
        /// If no `MediaType` can be guessed at the moment, the Rerun Viewer will try to guess one
        /// from the data at render-time. If it can't, rendering will fail with an error.
        static Result<AssetVideo> from_file(const std::filesystem::path& path);

        /// Creates a new `AssetVideo` from the given `bytes`.
        ///
        /// If no `MediaType` is specified, the Rerun Viewer will try to guess one from the data
        /// at render-time. If it can't, rendering will fail with an error.
        static AssetVideo from_bytes(
            rerun::Collection<uint8_t> bytes,
            std::optional<rerun::components::MediaType> media_type = {}
        ) {
            // TODO(jan): we could try and guess using magic bytes here, like rust does.
            AssetVideo asset = AssetVideo(std::move(bytes));
            asset.media_type = media_type;
            return asset;
        }

        static std::optional<rerun::components::MediaType> guess_media_type(
            const std::filesystem::path& path
        );

        // END of extensions from asset_video_ext.cpp, start of generated code:

      public:
        AssetVideo() = default;
        AssetVideo(AssetVideo&& other) = default;

        explicit AssetVideo(rerun::components::Blob _blob) : blob(std::move(_blob)) {}

        /// The Media Type of the asset.
        ///
        /// Supported values:
        /// * `video/mp4`
        ///
        /// If omitted, the viewer will try to guess from the data blob.
        /// If it cannot guess, it won't be able to render the asset.
        AssetVideo with_media_type(rerun::components::MediaType _media_type) && {
            media_type = std::move(_media_type);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::AssetVideo> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(const archetypes::AssetVideo& archetype
        );
    };
} // namespace rerun
