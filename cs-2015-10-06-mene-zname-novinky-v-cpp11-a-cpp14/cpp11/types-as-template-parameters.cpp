template <class T>
void func(T t) {}

// Unnamed type.
enum { e };

int main() {
    // Local type.
    struct A { int i; };
    A a;

    func(e); // OK.
    func(a); // OK.
}
