#include "thirdparty.h"

#include <iostream>
#include <memory>

int main() {
	auto p = std::make_unique<int>(1);

	int res;
	thirdparty_process("whatever", &res);

	std::cout << "res: " << res << '\n';
}
