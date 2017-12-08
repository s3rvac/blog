//
// Second example (structured bindings from C++17).
//

#include <iostream>
#include <tuple>
#include <type_traits>

int main() {
	{
		std::pair p{0, 0};
		auto [x, y] = p;
		y = 1;
		std::cout << p.first << ' ' << p.second << '\n'; // 0 0
	}

	{
		int a = 0, b = 0;
		auto [x, y] = std::tie(a, b);
		y = 1;
		std::cout << a << ' ' << b << '\n'; // 0 1
	}

	{
		// The above code block is roughly equivalent to
		int a = 0, b = 0;
		auto e = std::tie(a, b);
		decltype(std::get<0>(e)) x = std::get<0>(e);
		decltype(std::get<1>(e)) y = std::get<1>(e);
		y = 1;
		std::cout << a << ' ' << b << '\n'; // 0 1
	}
}
