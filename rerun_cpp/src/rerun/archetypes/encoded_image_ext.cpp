#include "../error.hpp"
#include "encoded_image.hpp"

#include "../collection_adapter_builtins.hpp"

#include <fstream>
#include <iostream>
#include <vector>

// It's undefined behavior to pre-declare std types, see http://www.gotw.ca/gotw/034.htm
// We want to use `std::filesystem::path`, so we have it include it in the header.
// <CODEGEN_COPY_TO_HEADER>

#include <filesystem>

// </CODEGEN_COPY_TO_HEADER>

// Uncomment for better auto-complete while editing the extension.
// #define EDIT_EXTENSION

namespace rerun::archetypes {

#ifdef EDIT_EXTENSION
    // <CODEGEN_COPY_TO_HEADER>

    /// Create a new `EncodedImage` from the contents of a file on disk, e.g. a PNG or JPEG.
    static Result<EncodedImage> from_file(const std::filesystem::path& filepath);

    /// Create a new `EncodedImage` from the contents of an image file, like a PNG or JPEG.
    ///
    /// If no `MediaType` is specified, the Rerun Viewer will try to guess one from the data
    /// at render-time. If it can't, rendering will fail with an error.
    static EncodedImage from_bytes(
        rerun::Collection<uint8_t> image_contents,
        std::optional<rerun::components::MediaType> media_type = {}
    ) {
        auto encoded_image = EncodedImage().with_blob(image_contents);
        if (media_type.has_value()) {
            return std::move(encoded_image).with_media_type(media_type.value());
        }
        return encoded_image;
    }

    // </CODEGEN_COPY_TO_HEADER>
#endif

    Result<EncodedImage> EncodedImage::from_file(const std::filesystem::path& filepath) {
        std::ifstream file(filepath, std::ios::binary);
        if (!file) {
            return Error(ErrorCode::FileRead, filepath.string());
        }

        // Get the size of the file:
        file.seekg(0, std::ios::end);
        auto file_size = file.tellg();
        file.seekg(0, std::ios::beg);

        std::vector<char> file_bytes(static_cast<size_t>(file_size));

        if (!file.read(file_bytes.data(), static_cast<std::streamsize>(file_size))) {
            return Error(ErrorCode::FileRead, filepath.string());
        }

        return EncodedImage::from_bytes(
            file_bytes,
            rerun::components::MediaType::guess_from_path(filepath)
        );
    }
} // namespace rerun::archetypes
