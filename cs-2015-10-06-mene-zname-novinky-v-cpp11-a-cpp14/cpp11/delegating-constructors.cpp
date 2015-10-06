class A {
public:
    A(): A(1) {} // Delegation to another constructor.
    A(int i): i(i) {}

private:
    int i;
};

int main() {
    A a;
}
