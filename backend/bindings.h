#if __cplusplus
#define EXTERN extern "C"
#else
#define EXTERN extern
#define false 0
#define true 1
typedef int bool; // or #define bool int
#endif

#if defined(_WIN32) || defined(_WIN64)
#if defined(INTERFACE_EXPORT)
#define EXPORT EXTERN __declspec(dllexport)
#else
#define EXPORT EXTERN __declspec(dllimport)
#endif
#else
#define EXPORT EXTERN
#endif

#include <stdint.h>
#include "deps/software-rendering/deps/sdl2/include/SDL_keycode.h"

typedef struct InitApp
{
    const char *title;
} InitApp;

enum AppEventType
{
    AppEventType_None,
    AppEventType_Quit,
    AppEventType_KeyDown,
    AppEventType_KeyUp,
    AppEventType_MouseDown,
    AppEventType_MouseUp,
    AppEventType_MouseMove,
    AppEventType_MouseWheel,
    AppEventType_WindowResize,
    AppEventType_TextInput,
};

enum Cursor
{
    Arrow,
    Hand,
    IBeam,
    Crosshair,
    HResize,
    VResize,
};

typedef struct AppEvent
{
    enum AppEventType type;
    int key;
    int x;
    int y;
    uint32_t code; // Stores extra infos like mods on key downs
    char *text;    // Text of textinput
} AppEvent;

// initApp is borrowed.
EXPORT int c_start_application(const InitApp *initApp);
// After that you can render things
EXPORT void c_pre_update_application();

// Ownership of AppEvent is transferred to the caller.
// Update application and return an event.
EXPORT AppEvent *c_poll_events();

EXPORT void c_delete_app_event(AppEvent *event);

EXPORT void c_set_cursor(enum Cursor cursor);

// Post rendering update of the application
EXPORT void c_post_update_application();
EXPORT void c_clean_up_editor();

EXPORT void c_draw_rect(float x, float y, float width, float height, float origin_x, float origin_y, float rotation, float corner_radius, bool fill, bool outline, uint32_t fill_color, uint32_t outline_color, float outline_thickness);
EXPORT void c_draw_circle(float x, float y, float radius, uint32_t color);
EXPORT void c_draw_circle_outline(float x, float y, float radius, float line_thickness, uint32_t color);
EXPORT void c_draw_line(float x1, float y1, float x2, float y2, float thickness, uint32_t color);
EXPORT void c_draw_text(unsigned int font, float x, float y, const char *text, uint32_t color, float outline_width, uint32_t outline_color);

// Everything with fonts
EXPORT unsigned int c_load_font(const char *path, int size);
EXPORT int c_font_get_line_height(unsigned int font);
EXPORT int c_font_get_line_top(unsigned int font);
EXPORT int c_font_get_line_bottom(unsigned int font);

EXPORT int c_font_get_text_width(unsigned int font, const char *text);
EXPORT int c_font_get_text_height(unsigned int font, const char *text);

// Paths
EXPORT void c_path_begin(bool stroke, bool fill, float stroke_width, uint32_t stroke_color, uint32_t fill_color);
EXPORT void c_path_end(bool closed);
EXPORT void c_path_move_to(float x, float y);
EXPORT void c_path_line_to(float x, float y);
EXPORT void c_path_cubic_bezier_curve_to(float cp1x, float cp1y, float cp2x, float cp2y, float x, float y, int segment_count);
EXPORT void c_path_quadr_bezier_curve_to(float cp1x, float cp1y, float x, float y, int segment_count);
EXPORT void c_path_ellips_arc_to(float x1, float y1, float angle, float radius_x, float radius_y, bool large_arc_flag, bool sweep_flag, int num_segments);
