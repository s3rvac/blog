#include <type_traits>

template <typename T>
class A {
    // C++11
    using TWithoutRefCpp11 = typename std::remove_reference<T>::type;

    // C++14
    using TWithoutRefCpp14 = std::remove_reference_t<T>;
};

int main() {}
