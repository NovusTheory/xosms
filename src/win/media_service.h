#pragma once

#include <napi.h>
#include <Windows.h>

using Context = Napi::Reference<Napi::Value>;
using DataType = std::string;
void CallJs(Napi::Env env, Napi::Function callback, Context *context, DataType *data);
using ButtonPressedEventCallbackFunction = Napi::TypedThreadSafeFunction<Context, DataType, CallJs>;
using FinalizerDataType = void;

class WindowsMediaService : public Napi::ObjectWrap<WindowsMediaService>
{
public:
    WindowsMediaService(const Napi::CallbackInfo &);
    ~WindowsMediaService();
    Napi::Value UpdateButtonEnablement(const Napi::CallbackInfo &);
    Napi::Value UpdateMediaProperties(const Napi::CallbackInfo &);
    Napi::Value UpdateMediaThumbnail(const Napi::CallbackInfo &);
    Napi::Value UpdateEvents(const Napi::CallbackInfo &);

    static Napi::Function GetClass(Napi::Env);

private:
    std::string _greeterName;
    ::Windows::Media::Playback::MediaPlayer ^ _mediaPlayer;
    Windows::Foundation::EventRegistrationToken _buttonPressedNativeHandlerRegistration;
    ButtonPressedEventCallbackFunction _buttonPressedCallback;
};
