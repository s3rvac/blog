#include <cstddef>
#include <string>

// Emulates the standard std::string literal ("..."s) from C++14. Since 's' is
// reserved by the standard, we have to use '_s' instead of 's'.
std::string operator "" _s(const char *str, std::size_t length) {
    return std::string(str, length);
}

int main() {
    std::string s1 = "abc\x00xyz";   // s1 contains "abc"
    std::string s2 = "abc\x00xyz"_s; // s1 contains "abc\x00xyz"
}
