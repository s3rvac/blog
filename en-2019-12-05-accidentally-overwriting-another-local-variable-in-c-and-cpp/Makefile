.PHONY: clean

all: example

example: thirdparty.o example.cpp
	$(CXX) -std=c++17 -pedantic -Wall -Wextra -g -o example example.cpp thirdparty.o

thirdparty.o: thirdparty.c thirdparty.h
	$(CC) -std=c11 -pedantic -Wall -Wextra -g -c -o thirdparty.o thirdparty.c

clean:
	rm -f example thirdparty.o
