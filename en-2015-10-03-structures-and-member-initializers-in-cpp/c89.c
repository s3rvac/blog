/* ANSI C (C89) code. */
/* */
/* gcc   -std=c89 -pedantic -fsyntax-only c89.c */
/* clang -std=c89 -pedantic -fsyntax-only c89.c */

struct A {
    int i;
    double j;
};

int main() {
    struct A a;            /* OK, but a.i and a.j have indeterminate values */
    struct A b = {};       /* OK, b.i is 0 and b.j is 0.0 */
    struct A c = {1};      /* OK, c.i is 1 and c.j is 0.0 */
    struct A d = {1, 2.0}; /* OK, d.i is 1 and d.j is 2.0*/
    return 0;
}
