#include <regex>
#include <string>

int main() {
    // No need to escape backslashes or quotes.
    // Matches e.g. 27:"c".
    std::regex pattern(R"(\d{1,3}:"[a-d]")");

    // Allows multi-line strings.
    std::string code(R"(
        int main() {
            return 0;
        }
    )");
}
