class Point {
public:
    constexpr Point(int x, int y): x(x), y(y) {}

    constexpr int getX() const { return x; }
    constexpr int getY() const { return y; }

private:
    int x;
    int y;
};

int main() {
    constexpr Point p(1, 1);
    auto x = p.getX();
}
