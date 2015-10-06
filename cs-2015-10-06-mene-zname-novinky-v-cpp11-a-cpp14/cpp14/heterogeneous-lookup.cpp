#include <functional>
#include <set>

struct A {
    explicit A(int i): i(i) {}

    int i;
};

inline bool operator<(const A &a, const int &i) { return a.i < i; }
inline bool operator<(const int &i, const A &a) { return i < a.i; }

int main() {
    std::set<A, std::less<>> s; // std::less<> is from C++14.

    // ...

    // OK, even when there is no way of implicitly converting 520 to A.
    auto it = s.find(520);
    if (it != s.end()) {
        // ...
    }
}
