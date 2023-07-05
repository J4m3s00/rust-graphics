#ifndef INTERFACE_EXPORT
#define INTERFACE_EXPORT
#endif

#include <stdint.h>
#include "bindings.h"
#include "pch.h"
#include <stdio.h>

#include "glad/glad.h"
#include "renderer/renderer.h"

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
EXPORT void c_draw_text(unsigned int font, float x, float y, const char *text, uint32_t color, float outline_width, uint32_t outline_color)
{
    sr::srDrawText(font, text, {x, y}, color, outline_width, outline_color);
}

EXPORT unsigned int c_load_font(const char *path, int size)
{
    return sr::srLoadFont(path, size);
}

EXPORT int c_font_get_line_height(unsigned int font)
{
    return sr::srFontGetLineHeight(font);
}

EXPORT int c_font_get_line_top(unsigned int font)
{
    return sr::srFontGetLineTop(font);
}

EXPORT int c_font_get_line_bottom(unsigned int font)
{
    return sr::srFontGetLineBottom(font);
}

EXPORT int c_font_get_text_width(unsigned int font, const char *text)
{
    return sr::srFontGetTextWidth(font, text);
}

EXPORT int c_font_get_text_height(unsigned int font, const char *text)
{
    return sr::srFontGetTextHeight(font, text);
}

EXPORT void c_path_begin(bool stroke, bool fill, float stroke_width, uint32_t stroke_color, uint32_t fill_color)
{
    sr::srBeginPath((stroke ? sr::PathType_Stroke : 0) | (fill ? sr::PathType_Fill : 0));
    sr::srPathSetStrokeEnabled(stroke);
    sr::srPathSetFillEnabled(fill);

    sr::srPathSetStrokeWidth(stroke_width);
    sr::srPathSetStrokeColor(stroke_color);
    sr::srPathSetFillColor(fill_color);
}

EXPORT void c_path_end(bool closed)
{
    sr::srEndPath(closed);
}

EXPORT void c_path_line_to(float x, float y)
{
    sr::srPathLineTo({x, y});
}

EXPORT void c_path_cubic_bezier_curve_to(float cp1x, float cp1y, float cp2x, float cp2y, float x, float y, int segment_count)
{
    sr::srPathCubicBezierTo({cp1x, cp1y}, {cp2x, cp2y}, {x, y}, segment_count);
}

EXPORT void c_path_quadr_bezier_curve_to(float cp1x, float cp1y, float x, float y, int segment_count)
{
    sr::srPathQuadraticBezierTo({cp1x, cp1y}, {x, y}, segment_count);
}

EXPORT void c_path_ellips_arc_to(float x1, float y1, float angle, float radius_x, float radius_y, bool large_arc_flag, bool sweep_flag, int num_segments)
{
    sr::srPathEllipticalArc({x1, y1}, angle, radius_x, radius_y, large_arc_flag, sweep_flag, num_segments);
}
