class A {
public:
    A(int i, int j): i(i), j(j) {}

private:
    int i;
    int j;
};

class B: public A {
    // C++98
    // B(int i, int j): A(i, j) {}

    // C++11
    using A::A;
};

int main() {
    B b(1, 2);
}
