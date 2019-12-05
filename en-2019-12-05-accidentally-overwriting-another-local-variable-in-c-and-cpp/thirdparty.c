#include "thirdparty.h"

#include <stdarg.h>

void thirdparty_process(const char* what, ...) {
	va_list argp;
	va_start(argp, what);
	// Here, the function should do what it is supposed to do based on the
	// value of `what`. For the purpose of illustration, assume that the
	// function only stores value 1 into a variable passed by a pointer.
	long *res = va_arg(argp, long*);
	*res = 1;
	va_end(argp);
}
