#pragma once

#include <stddef.h>

#ifdef _WIN32
#define RCF_API __declspec(dllimport)
#else
#define RCF_API
#endif

typedef struct rcf_forest rcf_forest;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

RCF_API rcf_forest *rcf_create(size_t dimensions);

RCF_API int rcf_set_param(rcf_forest *forest, const char *param, const char *value);

RCF_API void rcf_update(rcf_forest *forest, const float *point);

RCF_API double rcf_score(rcf_forest *forest, const float *point);

RCF_API void rcf_free(rcf_forest *forest);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
