typedef struct {

} VulkanAPI;

typedef struct {

} OpenGLAPI;

typedef struct {

} Window;

typedef GKBoolean char;

Window *gsfkCreateWindowWithOpenGL(const char* title,unsigned int width,unsigned int height,OpenGLAPI *api);
Window *gsfkCreateWindowWithVulkan(const char* title,unsigned int width,unsigned int height,OpenGLVulkan *api);

void gsfkShowWindow(Window *window);
void gsfkHideWindow(Window *window);
void gsfkSetMaximizeWindow(Window *window,GKBoolean b);
void gsfkSetMinimizeWindow(Window *window,GKBoolean b);

void gsfkGLMakeCurrent(OpenGLAPI *gl);
void gsfkGLSwapInterval(OpenGLAPI *gl,GKBoolean boolean);
void gsfkGLSwapBuffers(OpenGLAPI *gl);