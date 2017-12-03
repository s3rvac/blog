//
// Incorrect version number 1.
// When run, it either crashes or emits incorrect result.
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
	auto cs1 = x_n_times(2).c_str();
	auto cs2 = x_n_times(3).c_str();

	needs_c_strings(cs1, cs2);
}
