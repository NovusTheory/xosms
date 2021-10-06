#include "utils.h"

Napi::Error WinRtExceptionToJsError(Napi::Env env, Platform::Exception ^ exception)
{
    if (exception == nullptr)
    {
        return Napi::Error::New(env);
    }

    // Convert Platform::String to std::string for passing to Napi
    std::wstring errorMessageW(exception->Message->Begin());
    std::string errorMessageA(errorMessageW.begin(), errorMessageW.end());

    return Napi::Error::New(env, errorMessageA);

    /*Local<Value> error = Nan::Error(
        Nan::New<String>(reinterpret_cast<const uint16_t *>(errorMessage))
            .ToLocalChecked());
    Nan::Set(Nan::To<Object>(error).ToLocalChecked(),
             Nan::New<String>("HRESULT").ToLocalChecked(),
             Nan::New<Integer>(exception->HResult));*/
}

void ThrowWinRtExceptionInJs(Napi::Env env, Platform::Exception ^ exception)
{
    if (exception == nullptr)
    {
        return;
    }

    throw WinRtExceptionToJsError(env, exception);
}