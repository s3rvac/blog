#include <cstdio>

// New header files.
#include <ccomplex>
#include <cinttypes>
#include <cstdbool>
#include <cstdint>
#include <ctgmath>

// Support for variadic macros.
#define eprintf(...) std::fprintf(stderr, __VA_ARGS__)

int main() {
    // Type 'long long int'.
    long long int i = 1;

    // Macro __func__ that is replaced with the name of the current function
    // ("main" in this case).
    eprintf("Inside %s.\n", __func__);
}
