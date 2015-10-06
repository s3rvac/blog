struct A {
    int i = 0;
    double j = 0.0;
};

int main() {
    A a = {1};      // OK (C++14), compilation error in C++11.
    A b = {1, 2.0}; // OK (C++14), compilation error in C++11.
}
