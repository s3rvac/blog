#include <cstdlib>
#include <iostream>

// [[noreturn]] is a standard attribute indicating that the given function
// never returns to the caller.
[[noreturn]] void my_exit(int exit_code) {
    std::cerr << "exiting with " << exit_code << "\n";
    std::exit(exit_code);
}

int main() {
    my_exit(1);
}
