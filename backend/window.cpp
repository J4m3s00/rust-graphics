#ifndef INTERFACE_EXPORT
#define INTERFACE_EXPORT
#endif

#include "bindings.h"
#include "pch.h"
#include <stdio.h>

#include "glad/glad.h"
#include "SDL.h"
#include "renderer/renderer.h"
#include "intern.h"

struct AppState
{
    bool done = false;
    SDL_Window *window = NULL;
    int window_width = 1280;
    int window_height = 720;
    sr::Font font;
};

static AppState state;
static bool initialized = false;

EXPORT int
c_start_application(const InitApp *app)
{
    if (initialized)
    {
        printf("Application already started\n");
        return -1;
    }
    initialized = true;
    printf("Starting Application\n");
    // Setup SDL
    if (SDL_Init(SDL_INIT_EVERYTHING) != 0)
    {
        printf("Error: %s\n", SDL_GetError());
        return -1;
    }

    // Decide GL+GLSL versions
#if defined(IMGUI_IMPL_OPENGL_ES2)
    // GL ES 2.0 + GLSL 100
    const char *glsl_version = "#version 100";
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, 0);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_ES);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
#elif defined(__APPLE__)
    // GL 3.2 Core + GLSL 150
    const char *glsl_version = "#version 150";
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG); // Always required on Mac
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 2);
#else
    // GL 3.0 + GLSL 130
    const char *glsl_version = "#version 130";
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, 0);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
#endif

    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);
    SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8);
    SDL_WindowFlags window_flags = (SDL_WindowFlags)(SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE | SDL_WINDOW_ALLOW_HIGHDPI);
    SDL_Window *window = SDL_CreateWindow(app->title, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 1280, 720, window_flags);
    if (window == NULL)
    {
        printf("Error: %s\n", SDL_GetError());
        return -1;
    }
    SDL_GLContext gl_context = SDL_GL_CreateContext(window);
    SDL_GL_MakeCurrent(window, gl_context);
    SDL_GL_SetSwapInterval(1); // Enable vsync

    sr::srLoad((sr::SRLoadProc)SDL_GL_GetProcAddress);

    sr::Font font = sr::srLoadFont("/Users/lucaherzke/Documents/DEV/rust-ui/graphics/backend/deps/software-rendering/Roboto.ttf", 24);

    state.done = false;
    state.window = window;
    state.font = font;
    SDL_GL_GetDrawableSize(window, &state.window_width, &state.window_height);
    return 0;
}

EXPORT AppEvent *c_pre_update_application()
{
    AppEvent *result = new AppEvent();
#define RETURN_EXIT                              \
    AppEvent *ret = new AppEvent();              \
    ret->type = AppEventType::AppEventType_Quit; \
    return ret;

    SDL_Event event;
    while (SDL_PollEvent(&event))
    {
        switch (event.type)
        {
        case SDL_QUIT:
            state.done = true;
            break;
        case SDL_WINDOWEVENT:
            if (event.window.event == SDL_WINDOWEVENT_CLOSE && event.window.windowID == SDL_GetWindowID(state.window))
                result->type = AppEventType_Quit;
            else if (event.window.event == SDL_WINDOWEVENT_RESIZED)
            {
                state.window_width = event.window.data1;
                state.window_height = event.window.data2;
                SDL_GL_GetDrawableSize(state.window, &state.window_width, &state.window_height);
            }
            break;
        }
        if (event.type == SDL_QUIT)
            result->type = AppEventType_Quit;
        if (event.type == SDL_WINDOWEVENT && event.window.event == SDL_WINDOWEVENT_CLOSE && event.window.windowID == SDL_GetWindowID(state.window))
            result->type = AppEventType_Quit;
    }

    sr::srNewFrame(state.window_width, state.window_height);

    return result;
}

EXPORT void c_post_update_application()
{
    sr::srEndFrame();

    SDL_GL_SwapWindow(state.window);
}

EXPORT void c_clean_up_editor()
{
    sr::srTerminate();
    SDL_DestroyWindow(state.window);
    SDL_Quit();
}

sr::Font get_current_font()
{
    return state.font;
}

EXPORT float c_get_current_font_size()
{
    return state.font.Size;
}