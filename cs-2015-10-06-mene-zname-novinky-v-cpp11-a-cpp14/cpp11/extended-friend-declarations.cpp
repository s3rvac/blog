class A;
class B {
    friend class A; // Classic declaration (OK in both C++98 and C++11).
    friend A;       // Extended declaration (OK since C++11).
};

template <typename T>
class C {
    // friend class T; // Compilation error (even in C++98).
    friend T;          // Extended declaration (OK since C++11).
};

int main() {
    B b;
    C<int> c;
}
