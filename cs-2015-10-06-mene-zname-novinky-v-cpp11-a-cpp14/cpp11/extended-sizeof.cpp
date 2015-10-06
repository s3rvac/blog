#include <iostream>

struct A {
    int i;
};

int main() {
    A a;
    std::cout << sizeof(a.i) << "\n";  // OK (even in C++98).

    std::cout << sizeof(A::i) << "\n"; // Does not work in C++98, but OK in C++11.
}
