.PHONY: clean

all: intro example1 example2

intro: intro.cpp
	$(CXX) -std=c++11 -pedantic -O2 -o $@ $@.cpp

example1: example1.cpp
	$(CXX) -std=c++11 -pedantic -O2 -o $@ $@.cpp

example2: example2.cpp
	$(CXX) -std=c++17 -pedantic -O2 -o $@ $@.cpp

clean:
	rm -f intro example1 example2
