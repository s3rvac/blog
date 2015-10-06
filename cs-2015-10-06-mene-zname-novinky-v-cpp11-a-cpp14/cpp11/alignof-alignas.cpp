#include <iostream>

// Implementation-defined alignment.
struct A {
    char c;
    int i;
};

// Explicitly defined alignment (16 bytes).
struct alignas(16) B {
    char c;
    int i;
};

int main() {
    std::cout << alignof(A) << "\n"; // Prints an implementation-defined value (e.g. 4).
    std::cout << alignof(B) << "\n"; // Prints 16.
}
