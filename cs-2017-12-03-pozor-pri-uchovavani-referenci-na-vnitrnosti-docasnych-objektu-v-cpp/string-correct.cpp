//
// Correct version.
// When run, it emits correct result.
//

#include <cstring>
#include <iostream>
#include <string>

std::string x_n_times(std::size_t n) {
	return std::string(n, 'x');
}

void needs_c_strings(const char* cs1, const char* cs2) {
	std::cerr << std::strlen(cs1) + std::strlen(cs2) << '\n';
}

int main() {
	auto s1 = x_n_times(10);
	auto cs1 = s1.c_str();
	auto s2 = x_n_times(100);
	auto cs2 = s2.c_str();

	needs_c_strings(cs1, cs2);
}
