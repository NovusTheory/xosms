#include "media_service.h"

#include <iostream>
#include "utils.h"

WindowsMediaService::WindowsMediaService(const Napi::CallbackInfo &info) : ObjectWrap(info)
{
    Napi::Env env = info.Env();

    try
    {
        // This is deprecated and should be replaced with instantiating a MediaPlayer class (unknown how to get it working, never shows anything)
        _mediaPlayer = ::Windows::Media::Playback::BackgroundMediaPlayer::Current;
        _mediaPlayer->CommandManager->IsEnabled = false;

        ::Windows::Media::SystemMediaTransportControls ^ transportControls = _mediaPlayer->SystemMediaTransportControls;
        _buttonPressedNativeHandlerRegistration = transportControls->ButtonPressed += ref new ::Windows::Foundation::TypedEventHandler<::Windows::Media::SystemMediaTransportControls ^, ::Windows::Media::SystemMediaTransportControlsButtonPressedEventArgs ^>([this](::Windows::Media::SystemMediaTransportControls ^ sender, ::Windows::Media::SystemMediaTransportControlsButtonPressedEventArgs ^ args) {
            if (_buttonPressedCallback != nullptr)
            {
                std::string *button = new std::string("unknown");
                switch (args->Button)
                {
                case ::Windows::Media::SystemMediaTransportControlsButton::Play:
                {
                    button = new std::string("play");
                    break;
                }
                case ::Windows::Media::SystemMediaTransportControlsButton::Pause:
                {
                    button = new std::string("pause");
                    break;
                }
                case ::Windows::Media::SystemMediaTransportControlsButton::Previous:
                {
                    button = new std::string("previous");
                    break;
                }
                case ::Windows::Media::SystemMediaTransportControlsButton::Next:
                {
                    button = new std::string("next");
                    break;
                }
                }

                napi_status acquireStatus = _buttonPressedCallback.Acquire();
                if (acquireStatus == napi_ok)
                {
                    _buttonPressedCallback.BlockingCall(button);
                    _buttonPressedCallback.Release();
                }
            }
        });
    }
    catch (Platform::Exception ^ exception)
    {
        ThrowWinRtExceptionInJs(env, exception);
    }
}

WindowsMediaService::~WindowsMediaService()
{
    // Remove any bound button presses by aborting them (forces clean up once all threads have released it)
    if (_buttonPressedCallback != nullptr)
    {
        _buttonPressedCallback.Abort();
    }

    // Remove the native button handler
    ::Windows::Media::SystemMediaTransportControls ^ transportControls = _mediaPlayer->SystemMediaTransportControls;
    transportControls->ButtonPressed -= _buttonPressedNativeHandlerRegistration;
}

Napi::Value WindowsMediaService::UpdateButtonEnablement(const Napi::CallbackInfo &info)
{
    Napi::Env env = info.Env();

    if (info.Length() < 4)
    {
        Napi::TypeError::New(env, "Expected 4 arguments").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[0].IsBoolean())
    {
        Napi::TypeError::New(env, "Argument 1 was not a boolean").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[1].IsBoolean())
    {
        Napi::TypeError::New(env, "Argument 2 was not a boolean").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[2].IsBoolean())
    {
        Napi::TypeError::New(env, "Argument 3 was not a boolean").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[3].IsBoolean())
    {
        Napi::TypeError::New(env, "Argument 4 was not a boolean").ThrowAsJavaScriptException();
        return env.Null();
    }

    try
    {
        ::Windows::Media::SystemMediaTransportControls ^ transportControls = _mediaPlayer->SystemMediaTransportControls;
        transportControls->IsPlayEnabled = info[0].As<Napi::Boolean>();
        transportControls->IsPauseEnabled = info[1].As<Napi::Boolean>();
        transportControls->IsPreviousEnabled = info[2].As<Napi::Boolean>();
        transportControls->IsNextEnabled = info[3].As<Napi::Boolean>();
    }
    catch (Platform::Exception ^ exception)
    {
        ThrowWinRtExceptionInJs(env, exception);
    }

    return env.Null();
}

Napi::Value WindowsMediaService::UpdateMediaProperties(const Napi::CallbackInfo &info)
{
    Napi::Env env = info.Env();

    if (info.Length() < 6)
    {
        Napi::TypeError::New(env, "Expected 6 arguments").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[0].IsNumber())
    {
        Napi::TypeError::New(env, "Argument 1 was not a number").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[1].IsNumber())
    {
        Napi::TypeError::New(env, "Argument 2 was not a number").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[2].IsString())
    {
        Napi::TypeError::New(env, "Argument 3 was not a string").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[3].IsString())
    {
        Napi::TypeError::New(env, "Argument 4 was not a string").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[4].IsString())
    {
        Napi::TypeError::New(env, "Argument 5 was not a string").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[5].IsString())
    {
        Napi::TypeError::New(env, "Argument 6 was not a string").ThrowAsJavaScriptException();
        return env.Null();
    }

    try
    {
        ::Windows::Media::SystemMediaTransportControls ^ transportControls = _mediaPlayer->SystemMediaTransportControls;
        transportControls->PlaybackStatus = static_cast<::Windows::Media::MediaPlaybackStatus>(info[1].As<Napi::Number>().Int32Value());

        ::Windows::Media::SystemMediaTransportControlsDisplayUpdater ^ displayUpdater = transportControls->DisplayUpdater;
        displayUpdater->Type = static_cast<::Windows::Media::MediaPlaybackType>(info[0].As<Napi::Number>().Int32Value());
        displayUpdater->MusicProperties->Title = ref new Platform::String(reinterpret_cast<const wchar_t *>(info[2].As<Napi::String>().Utf16Value().c_str()));
        displayUpdater->MusicProperties->AlbumTitle = ref new Platform::String(reinterpret_cast<const wchar_t *>(info[3].As<Napi::String>().Utf16Value().c_str()));
        displayUpdater->MusicProperties->Artist = ref new Platform::String(reinterpret_cast<const wchar_t *>(info[4].As<Napi::String>().Utf16Value().c_str()));
        displayUpdater->MusicProperties->AlbumArtist = ref new Platform::String(reinterpret_cast<const wchar_t *>(info[5].As<Napi::String>().Utf16Value().c_str()));
        displayUpdater->Update();
    }
    catch (Platform::Exception ^ exception)
    {
        ThrowWinRtExceptionInJs(env, exception);
    }

    return env.Null();
}

Napi::Value WindowsMediaService::UpdateMediaThumbnail(const Napi::CallbackInfo &info)
{
    Napi::Env env = info.Env();

    if (info.Length() < 2)
    {
        Napi::TypeError::New(env, "Expected 2 arguments").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[0].IsNumber())
    {
        Napi::TypeError::New(env, "Argument 1 was not a number").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[1].IsString())
    {
        Napi::TypeError::New(env, "Argument 2 was not a string").ThrowAsJavaScriptException();
        return env.Null();
    }

    try
    {
        Napi::Number type = info[0].As<Napi::Number>();
        Napi::String thumbnail = info[1].As<Napi::String>();

        ::Windows::Storage::Streams::RandomAccessStreamReference ^ thumbnailStream;
        // TODO: Add file type thumbnail
        if (static_cast<int>(type) == 0)
        {
            // Silently ignore unknown thumbnail types
            return env.Null();
        }
        else if (static_cast<int>(type) == 2)
        {
            ::Windows::Foundation::Uri ^ uri = ref new ::Windows::Foundation::Uri(ref new Platform::String(reinterpret_cast<const wchar_t *>(thumbnail.Utf16Value().c_str())));
            thumbnailStream = ::Windows::Storage::Streams::RandomAccessStreamReference::CreateFromUri(uri);
        }
        else
        {
            Napi::TypeError::New(env, "Thumbnail type is not supported").ThrowAsJavaScriptException();
            return env.Null();
        }

        ::Windows::Media::SystemMediaTransportControls ^ transportControls = _mediaPlayer->SystemMediaTransportControls;
        ::Windows::Media::SystemMediaTransportControlsDisplayUpdater ^ displayUpdater = transportControls->DisplayUpdater;
        displayUpdater->Thumbnail = thumbnailStream;
        displayUpdater->Update();
    }
    catch (Platform::Exception ^ exception)
    {
        ThrowWinRtExceptionInJs(env, exception);
    }

    return env.Null();
}

Napi::Value WindowsMediaService::UpdateEvents(const Napi::CallbackInfo &info)
{
    Napi::Env env = info.Env();

    if (info.Length() < 1)
    {
        Napi::TypeError::New(env, "Expected 1 argument").ThrowAsJavaScriptException();
        return env.Null();
    }

    if (!info[0].IsFunction() && !info[0].IsNull())
    {
        Napi::TypeError::New(env, "Argument 1 was not a function or null").ThrowAsJavaScriptException();
        return env.Null();
    }

    try
    {
        Context *context = new Napi::Reference<Napi::Value>(Napi::Persistent(info.This()));
        if (_buttonPressedCallback != nullptr)
        {
            Napi::TypeError::New(env, "Button callback can only be set once and cannot be removed").ThrowAsJavaScriptException();
            return env.Null();
        }

        if (info[0].IsFunction())
        {
            _buttonPressedCallback = ButtonPressedEventCallbackFunction::New(
                env,
                info[0].As<Napi::Function>(),
                "xosms-buttonpressed-event-callback",
                0,
                1,
                context,
                [](Napi::Env, FinalizerDataType *, Context *fContext) {
                    delete fContext;
                });
        }
    }
    catch (Platform::Exception ^ exception)
    {
        ThrowWinRtExceptionInJs(env, exception);
    }

    return env.Null();
}

void CallJs(Napi::Env env, Napi::Function callback, Context *context,
            DataType *data)
{
    // Check if the JS environment is still available
    if (env != nullptr)
    {
        // Ensure a callback is provided
        if (callback != nullptr)
        {
            callback.Call(context->Value(), {Napi::String::New(env, *data)});
        }
    }
    if (data != nullptr)
    {
        // Delete all the data passed
        delete data;
    }
}

Napi::Function WindowsMediaService::GetClass(Napi::Env env)
{
    return DefineClass(env, "WindowsMediaService", {
                                                       WindowsMediaService::InstanceMethod("UpdateButtonEnablement", &WindowsMediaService::UpdateButtonEnablement),
                                                       WindowsMediaService::InstanceMethod("UpdateMediaProperties", &WindowsMediaService::UpdateMediaProperties),
                                                       WindowsMediaService::InstanceMethod("UpdateMediaThumbnail", &WindowsMediaService::UpdateMediaThumbnail),
                                                       WindowsMediaService::InstanceMethod("UpdateEvents", &WindowsMediaService::UpdateEvents),
                                                   });
}