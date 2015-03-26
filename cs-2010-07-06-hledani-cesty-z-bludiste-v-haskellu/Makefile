#
# Project:   A very simple maze solver
# Copyright: (c) 2010 by Petr Zemek <s3rvac@gmail.com>
# License:   BSD, see LICENSE for more details
#
# Usage:
#   - compile project:       make
#   - clean:                 make clean
#   - pack (zip):            make pack
#

# Project
PROJNAME = maze

# Compiler
HC = ghc
HCFLAGS = -Wall --make

# Project compilation
all: Main.hs
	$(HC) $(HCFLAGS) Main.hs -o $(PROJNAME)

# Cleaning
clean:
	rm -rf *.o *.hi
	rm -f $(PROJNAME)

# Packing
pack:
	zip -r maze-solver.zip README.md Makefile Main.hs example*.txt
