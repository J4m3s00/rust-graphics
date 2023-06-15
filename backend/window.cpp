#ifndef INTERFACE_EXPORT
#define INTERFACE_EXPORT
#endif

#include "bindings.h"
#include "pch.h"
#include <stdio.h>

#include "glad/glad.h"
#include "SDL.h"
#include "renderer/renderer.h"

struct AppState
{
    bool done = false;
    SDL_Window *window = NULL;
    int window_width = 1280;
    int window_height = 720;
    int draw_width = 1280;
    int draw_height = 720;
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

    state.window = window;
    SDL_GL_GetDrawableSize(window, &state.draw_width, &state.draw_height);
    SDL_GetWindowSize(window, &state.window_width, &state.window_height);
    return 0;
}

EXPORT void c_pre_update_application()
{
    sr::srNewFrame(state.draw_width, state.draw_height, state.window_width, state.window_height);
}

EXPORT AppEvent *c_poll_events()
{
    SDL_Event event;
    if (!SDL_PollEvent(&event))
    {
        return NULL;
    }
    AppEvent *result = new AppEvent();

    switch (event.type)
    {
    case SDL_QUIT:
        result->type = AppEventType_Quit;
        break;
    case SDL_WINDOWEVENT:
        if (event.window.event == SDL_WINDOWEVENT_CLOSE && event.window.windowID == SDL_GetWindowID(state.window))
            result->type = AppEventType_Quit;
        else if (event.window.event == SDL_WINDOWEVENT_RESIZED)
        {
            SDL_GL_GetDrawableSize(state.window, &state.draw_width, &state.draw_height);
            SDL_GetWindowSize(state.window, &state.window_width, &state.window_height);
            result->type = AppEventType_WindowResize;
            result->x = state.window_width;
            result->y = state.window_height;
        }
        break;
    case SDL_KEYDOWN:
        result->type = AppEventType_KeyDown;
        result->key = event.key.keysym.sym;
        result->code = event.key.keysym.mod;
        break;
    case SDL_KEYUP:
        result->type = AppEventType_KeyUp;
        result->key = event.key.keysym.sym;
        result->code = event.key.keysym.mod;
        break;
    case SDL_MOUSEBUTTONDOWN:
        result->type = AppEventType_MouseDown;
        result->key = event.button.button;
        result->x = event.button.x;
        result->y = event.button.y;
        break;
    case SDL_MOUSEBUTTONUP:
        result->type = AppEventType_MouseUp;
        result->key = event.button.button;
        result->x = event.button.x;
        result->y = event.button.y;
        break;
    case SDL_MOUSEMOTION:
        result->type = AppEventType_MouseMove;
        result->x = event.motion.x;
        result->y = event.motion.y;
        break;
    }

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
