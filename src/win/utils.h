#pragma once

#include <napi.h>
#include <Windows.h>

Napi::Error WinRtExceptionToJsError(Napi::Env env, Platform::Exception ^ exception);
void ThrowWinRtExceptionInJs(Napi::Env env, Platform::Exception ^ exception);
