//
// Copyright: (c) 2015 by Petr Zemek <s3rvac@gmail.com>
// License:   BSD, see LICENSE for more details
//

#include <map>
#include <string>

class Person {
public:
    Person(const std::string &name, int age):
        name(name), age(age) {}

    // Person(const Person &) = delete;
    // Person(Person &&) = delete;

    // ...

private:
    std::string name;
    int age;
};

int main() {
    std::map<int, Person> m;

    // Needs Person::Person(const Person &) or Person::Person(Person &&).
    // C++11 only
    m.emplace(1, Person("Fred Astaire", 88));

    // Does not need Person::Person(const Person &) or Person::Person(Person &&).
    // C++11 only
    m.emplace(
        std::piecewise_construct,
        std::forward_as_tuple(1),
        std::forward_as_tuple("Fred Astaire", 88)
    );
}
