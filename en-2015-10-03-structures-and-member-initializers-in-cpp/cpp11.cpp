// C++11 code with initializers.
//
// g++     -std=c++11 -pedantic -fsyntax-only cpp11.cpp
// clang++ -std=c++11 -pedantic -fsyntax-only cpp11.cpp

struct A {
    int i = 0;
    double j = 0.0;
};

int main() {
    A a;            // OK, a.i is 0 and a.j is 0.0
    A b = {};       // OK, b.i is 0 and b.j is 0.0
    A c = {1};      // FAIL (compilation error)
    A d = {1, 2.0}; // FAIL (compilation error)
    // GCC:
    //   error: could not convert ‘{1}’ from ‘<brace-enclosed initializer list>’ to ‘A’
    //   error: could not convert ‘{1, 2.0e+0}’ from ‘<brace-enclosed initializer list>’ to ‘A’
    //
    // Clang:
    //   error: no matching constructor for initialization of 'A'
    return 0;
}
