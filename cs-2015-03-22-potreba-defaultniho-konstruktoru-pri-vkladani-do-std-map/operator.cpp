//
// Copyright: (c) 2015 by Petr Zemek <s3rvac@gmail.com>
// License:   BSD, see LICENSE for more details
//

#include <map>
#include <string>

class Person {
public:
    Person(): age(0) {} // Because of m[1] below.

    Person(const std::string &name, int age):
        name(name), age(age) {}

    // ...

private:
    std::string name;
    int age;
};

int main() {
    std::map<int, Person> m;

    // Needs Person::Person().
    m[1] = Person("Fred Astaire", 88);
}
