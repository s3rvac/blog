//
// Correct version.
// When run, it emits correct result.
//

#include <iostream>
#include <vector>

std::vector<int> foo() {
	return {1, 2, 3, 4, 5};
}

int main() {
	// Variant 1.
	auto v = foo();
	for (auto i = v.begin(), e = v.end(); i != e; ++i) {
		std::cerr << *i << '\n';
	}

	// Variant 2 (recommended).
	for (auto x : foo()) {
		std::cerr << x << '\n';
	}
}
