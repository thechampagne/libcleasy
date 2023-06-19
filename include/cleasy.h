/*
 * Copyright (c) 2023 XXIV
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
#ifndef __CLEASY_H__
#define __CLEASY_H__

#ifdef __cplusplus
extern "C" {
#endif

typedef struct cleasy_app_t cleasy_app_t;

extern cleasy_app_t* cleasy_app_new(const char* name, const char* version, const char* author);

extern int cleasy_app_arg_was_used(const cleasy_app_t* app, const char* arg);

extern int cleasy_app_add_arg(cleasy_app_t* app, const char* name, const char* help, const char* data);

extern char* cleasy_app_get_arg_data(const cleasy_app_t* app, const char* name, int* is_err);

extern int cleasy_app_version_is(const cleasy_app_t* app);

extern int cleasy_app_help_is(const cleasy_app_t* app);

extern char* cleasy_app_version_info(const cleasy_app_t* app);

extern char* cleasy_app_help_info(const cleasy_app_t* app);

extern void cleasy_app_destroy(cleasy_app_t* app);

extern void cleasy_string_destroy(char* ptr);

#ifdef __cplusplus
}
#endif

#endif // __CLEASY_H__
