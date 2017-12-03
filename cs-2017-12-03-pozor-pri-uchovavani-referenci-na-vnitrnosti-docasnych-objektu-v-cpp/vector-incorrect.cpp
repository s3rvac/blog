//
// Incorrect version.
// When run, it either crashes or emits invalid result.
//

#include <iostream>
#include <vector>

std::vector<int> foo() {
	return {1, 2, 3, 4, 5};
}

int main() {
	for (auto i = foo().begin(), e = foo().end(); i != e; ++i) {
		std::cerr << *i << '\n';
	}
}
