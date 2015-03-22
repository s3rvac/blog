#
# Copyright: (c) 2015 by Petr Zemek <s3rvac@gmail.com>
# License:   BSD, see LICENSE for more details
#

.PHONY: all clean

CXXFLAGS=-std=c++11 -pedantic -Wall -Wextra
PROGS=operator insert emplace

all: $(PROGS)

operator: operator.cpp
	$(CXX) $(CXXFLAGS) -o $@ $^

insert: insert.cpp
	$(CXX) $(CXXFLAGS) -o $@ $^

emplace: emplace.cpp
	$(CXX) $(CXXFLAGS) -o $@ $^

clean:
	rm -f $(PROGS)
