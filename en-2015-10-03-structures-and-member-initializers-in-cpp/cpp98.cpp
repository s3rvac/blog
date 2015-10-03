// C++98 code.
//
// g++     -std=c++98 -pedantic -fsyntax-only cpp98.cpp
// clang++ -std=c++98 -pedantic -fsyntax-only cpp98.cpp

struct A {
    int i;
    double j;
};

int main() {
    A a;            // OK, but a.i and a.j have indeterminate values
    A b = {};       // OK, b.i is 0 and b.j is 0.0
    A c = {1};      // OK, c.i is 1 and c.j is 0.0
    A d = {1, 2.0}; // OK, d.i is 1 and d.j is 2.0
    return 0;
}
