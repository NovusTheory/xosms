#pragma once

#include <napi.h>

#ifdef _WIN32
#include "win/media_service.h"
// Used so we can initialize COM
#include "combaseapi.h"
#endif
