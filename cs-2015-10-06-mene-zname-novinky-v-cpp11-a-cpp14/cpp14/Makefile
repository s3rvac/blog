#
# Copyright: (c) 2015 by Petr Zemek <s3rvac@gmail.com>
# License:   BSD, see LICENSE for more details
#

.PHONY: all clean

SOURCES=$(wildcard *.cpp)
PROGS=$(patsubst %.cpp,%,$(SOURCES))
CXXFLAGS=-std=c++14 -pedantic -Wall -Wextra -Wno-unused-variable

all: $(PROGS)

%: %.cpp
	$(CXX) $(CXXFLAGS) -o $@ $^

clean:
	rm -f $(PROGS)
