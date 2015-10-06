#include <vector>

// This says that an explicit instantiation of the following template must be
// provided either in a different translation unit, or later in the same
// translation unit.
extern template class std::vector<int>;

// An explicit instantiation of the template (a construct available since
// C++98).
template class std::vector<int>;

int main() {
    std::vector<int> v;
}
