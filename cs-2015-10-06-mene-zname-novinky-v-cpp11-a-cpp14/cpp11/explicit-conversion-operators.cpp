class A {
public:
    // Explicit conversion operator.
    explicit operator bool() const {
        return false;
    }
};

void func(int i);

int main() {
    A a;
    if (a) { /* ... */ } // OK, bool context.
    // func(a);          // Would fail to compile (operator bool is explicit).
}
