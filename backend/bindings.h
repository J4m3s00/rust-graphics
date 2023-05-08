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
};

typedef struct AppEvent
{
    enum AppEventType type;
    int key;
    int x;
    int y;
    uint32_t code; // Stores extra infos like mods on key downs
} AppEvent;

// initApp is borrowed.
EXPORT int c_start_application(const InitApp *initApp);
// After that you can render things
EXPORT void c_pre_update_application();

// Ownership of AppEvent is transferred to the caller.
// Update application and return an event.
EXPORT AppEvent *c_poll_events();

// Post rendering update of the application
EXPORT void c_post_update_application();
EXPORT void c_clean_up_editor();

EXPORT void c_draw_rect(float x, float y, float width, float height, float origin_x, float origin_y, float rotation, float corner_radius, bool fill, bool outline, uint32_t fill_color, uint32_t outline_color, float outline_thickness);
EXPORT void c_draw_circle(float x, float y, float radius, uint32_t color);
EXPORT void c_draw_circle_outline(float x, float y, float radius, float line_thickness, uint32_t color);
EXPORT void c_draw_line(float x1, float y1, float x2, float y2, float thickness, uint32_t color);
EXPORT void c_draw_text(float x, float y, const char *text, uint32_t color);

EXPORT float c_get_current_font_size();