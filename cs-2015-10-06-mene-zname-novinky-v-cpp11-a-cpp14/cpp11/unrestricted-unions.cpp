// A structure with a user-provided constructor.
struct Range {
    Range(int start, int end): start(start), end(end) {}
    int start;
    int end;
};

// A union containing a member object with a user-provided constructor.
union U {
    int i;
    Range r; // Forbidden in C++98, OK in C++11.

    // Due to the Range member, we need to explicitly define a constructor.
    U(): r(0, 0) {}
};

int main() {
    U u;
}
