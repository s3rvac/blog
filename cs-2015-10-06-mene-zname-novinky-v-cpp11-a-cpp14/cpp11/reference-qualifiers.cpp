#include <iostream>

class A {
public:
    void foo()  & { std::cout << "foo() &\n"; }
    void foo() && { std::cout << "foo() &&\n"; }
};

int main() {
    A a;
    a.foo();   // Prints "foo() &" because 'a' is an l-value.
    A().foo(); // Prints "foo() &&" because 'A()' is an r-value.
}
