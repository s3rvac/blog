//
// First example (proxy objects).
//

#include <iostream>
#include <vector>

int main() {
	{
		std::vector<int> v{0, 0, 0};
		auto x = v[1];
		x = 1;
		std::cout << v[0] << ' ' << v[1] << ' ' << v[2] << '\n'; // 0 0 0
	}

	{
		std::vector<bool> v{0, 0, 0};
		auto x = v[1];
		x = 1;
		std::cout << v[0] << ' ' << v[1] << ' ' << v[2] << '\n'; // 0 1 0
	}
}
