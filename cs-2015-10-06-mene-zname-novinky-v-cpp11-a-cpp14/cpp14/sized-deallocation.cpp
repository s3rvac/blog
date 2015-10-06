int main() {
    int *p = new int;
    ::operator delete(p, sizeof(int));
}
