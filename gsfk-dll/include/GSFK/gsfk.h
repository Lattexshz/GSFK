/* GSFK - GraphicS FrameworK
 * MIT License
 *
 * Copyright (c) 2023 Lattexshz
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

 #pragma once

typedef struct {

} VulkanAPI;

typedef struct {

} OpenGLAPI;

typedef struct {

} Window;

typedef char GKBoolean;

extern "C" {
    Window *gsfkCreateWindowWithOpenGL(const char* title,unsigned int width,unsigned int height,OpenGLAPI *api);
    Window *gsfkCreateWindowWithVulkan(const char* title,unsigned int width,unsigned int height,VulkanAPI *api);

    void gsfkShowWindow(Window *window);
    void gsfkHideWindow(Window *window);
    void gsfkSetMaximizeWindow(Window *window,GKBoolean b);
    void gsfkSetMinimizeWindow(Window *window,GKBoolean b);

    void gsfkGLMakeCurrent(OpenGLAPI *gl);
    void gsfkGLSwapInterval(OpenGLAPI *gl,GKBoolean boolean);
    void gsfkGLSwapBuffers(OpenGLAPI *gl);

    void gsfkRunWindow(Window* window);

    // Callback setters
    void gsfkSetUpdatedCallback(Window *window,void (* callback)());
    void gsfkSetRedrawRequestedCallback(Window *window,void (* callback)());
    void gsfkSetCloseRequestedCallback(Window *window,void (* callback)());
}