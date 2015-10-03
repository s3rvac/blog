// C++14 code with initializers.
//
// g++     -std=c++14 -pedantic -fsyntax-only cpp14.cpp
// clang++ -std=c++14 -pedantic -fsyntax-only cpp14.cpp

struct A {
    int i = 0;
    double j = 0.0;
};

int main() {
    A a;            // OK, a.i is 0 and a.j is 0.0
    A b = {};       // OK, b.i is 0 and b.j is 0.0
    A c = {1};      // OK, c.i is 1 and c.j is 0.0
    A d = {1, 2.0}; // OK, d.i is 1 and d.j is 2.0
    // Note: You need GCC >= 5 to compile this code because GCC 4.9 does not
    //       support this C++14 feature (the compilation will fail).
    return 0;
}
