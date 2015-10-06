namespace A {
    inline namespace B {
        class C {};
    }
}

int main() {
    // Either of these two qualifications is OK because B is an inline
    // namespace.
    A::B::C c1;
    A::C c2;
}
