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
    // C++11
    m.insert({1, Person("Fred Astaire", 88)});
    // C++98
    m.insert(std::map<int, Person>::value_type(1, Person("Fred Astaire", 88)));
    // or
    m.insert(std::make_pair(1, Person("Fred Astaire", 88)));
}
