//
// Introductory example that illustrates the difference between auto and auto&.
//

#include <iostream>

int main() {
	{
		int x = 0;
		auto y = x;
		y = 1;
		std::cout << x << ' ' << y << '\n'; // 0 0
	}

	{
		int x = 0;
		auto& y = x;
		y = 1;
		std::cout << x << ' ' << y << '\n'; // 0 1
	}
}
