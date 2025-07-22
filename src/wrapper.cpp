#include "wrapper.h"
#include "../ext/MicroTeX/src/latex.h"
#include "../ext/MicroTeX/src/render.h"
#include "../ext/MicroTeX/src/graphic/graphic.h"
#include "../ext/MicroTeX/src/graphic/graphic_basic.h"
#include "../ext/MicroTeX/src/platform/cairo/graphic_cairo.h"

#include <cairo/cairo.h>
#include <cairomm/context.h>
#include <cairomm/surface.h>
#include <string>
#include <cstdlib>
#include <memory>

using namespace tex;

// Helper to convert UTF-8 to wide string
static std::wstring utf8_to_wstring(const std::string& utf8) {
    // Use mbstowcs as a replacement for deprecated wstring_convert
    size_t len = utf8.length() + 1;
    std::wstring wstr(len, L'\0');
    size_t converted = mbstowcs(&wstr[0], utf8.c_str(), len);
    if (converted == static_cast<size_t>(-1)) {
        return L""; // Conversion failed
    }
    wstr.resize(converted);
    return wstr;
}

extern "C" {

void microtex_init() {
    LaTeX::init();
}

void microtex_init_with_path(const char* res_path) {
    LaTeX::init(std::string(res_path));
}

void microtex_release() {
    LaTeX::release();
}

void microtex_set_debug(int debug) {
    LaTeX::setDebug(debug != 0);
}

TeXRender_Handle microtex_parse(
    const char* latex_utf8,
    int width,
    float text_size,
    float line_space,
    unsigned int color
) {
    try {
        std::string utf8_str(latex_utf8);
        std::wstring wide_str = utf8_to_wstring(utf8_str);
        
        TeXRender* render = LaTeX::parse(
            wide_str,
            width,
            text_size,
            line_space,
            color
        );
        
        return static_cast<TeXRender_Handle>(render);
    } catch (...) {
        return nullptr;
    }
}

int microtex_render_get_width(TeXRender_Handle handle) {
    if (!handle) return 0;
    TeXRender* render = static_cast<TeXRender*>(handle);
    return render->getWidth();
}

int microtex_render_get_height(TeXRender_Handle handle) {
    if (!handle) return 0;
    TeXRender* render = static_cast<TeXRender*>(handle);
    return render->getHeight();
}

int microtex_render_get_depth(TeXRender_Handle handle) {
    if (!handle) return 0;
    TeXRender* render = static_cast<TeXRender*>(handle);
    return render->getDepth();
}

float microtex_render_get_baseline(TeXRender_Handle handle) {
    if (!handle) return 0.0f;
    TeXRender* render = static_cast<TeXRender*>(handle);
    return render->getBaseline();
}

void microtex_render_draw_to_buffer(
    TeXRender_Handle handle,
    unsigned char* buffer,
    int buffer_width,
    int buffer_height,
    int x,
    int y
) {
    if (!handle || !buffer) return;
    
    TeXRender* render = static_cast<TeXRender*>(handle);
    
    // Create a Cairo surface from the buffer
    cairo_surface_t* c_surface = cairo_image_surface_create_for_data(
        buffer,
        CAIRO_FORMAT_ARGB32,
        buffer_width,
        buffer_height,
        buffer_width * 4
    );
    
    // Create cairomm surface wrapper
    Cairo::RefPtr<Cairo::Surface> surface = 
        Cairo::RefPtr<Cairo::Surface>(new Cairo::Surface(c_surface));
    
    // Create cairomm context
    Cairo::RefPtr<Cairo::Context> context = Cairo::Context::create(surface);
    
    // Create Graphics2D_cairo instance
    Graphics2D_cairo g2(context);
    
    // Draw the formula
    render->draw(g2, x, y);
    
    // Ensure all drawing operations are flushed
    context->show_page();
    surface->flush();
    
    // The surface will be cleaned up by RefPtr
}

void microtex_render_free(TeXRender_Handle handle) {
    if (handle) {
        TeXRender* render = static_cast<TeXRender*>(handle);
        delete render;
    }
}

} // extern "C"