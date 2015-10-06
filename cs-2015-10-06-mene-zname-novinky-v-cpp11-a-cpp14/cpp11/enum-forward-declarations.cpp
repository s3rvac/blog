enum A : short;        // OK, base type is short.
// enum B;             // Compilation error (base type is missing).
enum class C : short;  // OK (scoped enum), base type is short.
enum class D;          // OK (scoped enum), base type is implicitly int.

int main() {}
