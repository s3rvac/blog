// Specifies that foo() should never throw an exception.
void foo() noexcept {}

int main() {
    foo();
}
