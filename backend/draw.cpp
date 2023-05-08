#ifndef INTERFACE_EXPORT
#define INTERFACE_EXPORT
#endif

#include <stdint.h>
#include "bindings.h"
#include "pch.h"
#include <stdio.h>

#include "glad/glad.h"
#include "renderer/renderer.h"

#include "intern.h"

EXPORT void c_draw_rect(float x, float y, float width, float height, float origin_x, float origin_y, float rotation, float corner_radius, bool fill, bool outline, uint32_t fill_color, uint32_t outline_color, float outline_thickness)
{
    sr::PathType path_type = 0;
    if (fill)
        path_type |= sr::PathType_Fill;
    if (outline)
        path_type |= sr::PathType_Stroke;

    sr::PathStyle path_style = {};
    path_style.FillColor = fill_color;
    path_style.StrokeColor = outline_color;
    path_style.StrokeWidth = outline_thickness;

    sr::srDrawRectanglePro({x, y}, {origin_x, origin_y, width, height}, rotation, corner_radius, path_type, path_style);
}

EXPORT void c_draw_circle(float x, float y, float radius, uint32_t color)
{
    sr::srDrawCircle({x, y}, radius, color);
}

EXPORT void c_draw_circle_outline(float x, float y, float radius, float line_thickness, uint32_t color)
{
    sr::srDrawCircleOutline({x, y}, radius, line_thickness, color);
}


EXPORT void c_draw_line(float x1, float y1, float x2, float y2, float thickness, uint32_t color)
{
    sr::srBeginPath(sr::PathType_Stroke);
    sr::srPathSetStrokeColor(color);
    sr::srPathSetStrokeWidth(thickness);
    sr::srPathLineTo({x1, y1});
    sr::srPathLineTo({x2, y2});
    sr::srEndPath();
}
EXPORT void c_draw_text(float x, float y, const char *text, uint32_t color)
{
    sr::srDrawText(get_current_font(), text, {x, y}, color);
}