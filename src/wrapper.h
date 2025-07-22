#ifndef MICROTEX_WRAPPER_H
#define MICROTEX_WRAPPER_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle types
typedef void* MicroTeX_Handle;
typedef void* TeXRender_Handle;

// Initialize MicroTeX with default resources
void microtex_init();

// Initialize MicroTeX with custom resource path
void microtex_init_with_path(const char* res_path);

// Release MicroTeX resources
void microtex_release();

// Set debug mode
void microtex_set_debug(int debug);

// Parse LaTeX string (UTF-8) and create a render handle
// Returns NULL on error
TeXRender_Handle microtex_parse(
    const char* latex_utf8,
    int width,
    float text_size,
    float line_space,
    unsigned int color
);

// Get render dimensions
int microtex_render_get_width(TeXRender_Handle handle);
int microtex_render_get_height(TeXRender_Handle handle);
int microtex_render_get_depth(TeXRender_Handle handle);
float microtex_render_get_baseline(TeXRender_Handle handle);

// Draw to a buffer (RGBA format)
void microtex_render_draw_to_buffer(
    TeXRender_Handle handle,
    unsigned char* buffer,
    int buffer_width,
    int buffer_height,
    int x,
    int y
);

// Free a render handle
void microtex_render_free(TeXRender_Handle handle);

#ifdef __cplusplus
}
#endif

#endif // MICROTEX_WRAPPER_H