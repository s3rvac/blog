.PHONY: clean

CXX=g++
CXXFLAGS=-std=c++14 -pedantic -Wall -Wextra -O2 -g

all: string-incorrect1 string-incorrect2 string-correct vector-incorrect vector-correct

string-incorrect1: string-incorrect1.cpp
	$(CXX) $(CXXFLAGS) $@.cpp -o $@

string-incorrect2: string-incorrect2.cpp
	$(CXX) $(CXXFLAGS) $@.cpp -o $@

string-correct: string-correct.cpp
	$(CXX) $(CXXFLAGS) $@.cpp -o $@

vector-incorrect: vector-incorrect.cpp
	$(CXX) $(CXXFLAGS) $@.cpp -o $@

vector-correct: vector-correct.cpp
	$(CXX) $(CXXFLAGS) $@.cpp -o $@

clean:
	rm -f string-incorrect1 string-incorrect2 string-correct vector-incorrect vector-correct
