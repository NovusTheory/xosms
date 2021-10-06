#include "xosms.h"

Napi::Object Init(Napi::Env env, Napi::Object exports)
{
#ifdef _WIN32
    // Initialize WinRT
    CoInitializeEx(nullptr, COINIT_MULTITHREADED);

    Napi::String name = Napi::String::New(env, "WindowsMediaService");
    exports.Set(name, WindowsMediaService::GetClass(env));
#endif
    return exports;
}

NODE_API_MODULE(addon, Init)
