#include "mat4x4.hpp"
#include "vec4d.hpp"

// Uncomment for better auto-complete while editing the extension.
// #define EDIT_EXTENSION

namespace rerun {
    namespace datatypes {

#ifdef EDIT_EXTENSION
        struct Mat4x4Ext {
            float coeffs[9];

#define Mat4x4 Mat4x4Ext
            // [CODEGEN COPY TO HEADER START]

            static const Mat4x4 IDENTITY;

            /// Creates a new 4x4 matrix from 3 *columns* of 4 elements each.
            Mat4x4(const Vec4D (&columns)[4])
                : flat_columns{
                      columns[0].x(),
                      columns[0].y(),
                      columns[0].z(),
                      columns[0].w(),
                      columns[1].x(),
                      columns[1].y(),
                      columns[1].z(),
                      columns[1].w(),
                      columns[2].x(),
                      columns[2].y(),
                      columns[2].z(),
                      columns[2].w(),
                      columns[3].x(),
                      columns[3].y(),
                      columns[3].z(),
                      columns[3].w(),
                  } {}

            // [CODEGEN COPY TO HEADER END]
        };

#undef Mat4x4
#else
#define Mat4x4Ext Mat4x4
#endif

        const Mat4x4Ext Mat4x4Ext::IDENTITY = Mat4x4Ext({
            {1.0f, 0.0f, 0.0f, 0.0f},
            {0.0f, 1.0f, 0.0f, 0.0f},
            {0.0f, 0.0f, 1.0f, 0.0f},
            {0.0f, 0.0f, 0.0f, 1.0f},
        });

    } // namespace datatypes
} // namespace rerun
