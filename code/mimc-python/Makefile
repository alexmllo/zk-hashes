#### BEGIN USER OPTIONS ####
# Enables debug mode: 0 (no debug) 1 (debug) 2 (debug with optimization)
DEBUG := 1
# Enable micro-benchmarking during tests, useful to check performance variation across builds: 0/1
MEASURE_PERFORMANCE := 1
# Enable OpenMP parallelization: O/1
MULTICORE := 0
# Override default compiler and flags: 0/1
OVERRIDE_DEFAULT_CC := 1
OVERRIDE_DEFAULT_CXX := 1
# Enable assembly code (improves performance, might be buggy): 0/1
USE_ASM := 0
#### END USER OPTIONS ####


#### BEGIN TARGETS ####
# C++ Targets which only have tests
TARGETS_ONLYTEST_CXX :=

# C++ Targets which have tests and an additional executable (e.g. benchmarks)
TARGETS_TEST_CXX :=

# C++ Targets which do not have tests
TARGETS_NOTEST_CXX :=

# C++ Targets which are not executables
TARGETS_NOEXE_CXX :=
TARGETS_NOEXE_CXX += rand

# C Targets which only have tests
TARGETS_ONLYTEST_CC :=

# C Targets which have tests and an additional executable (e.g. benchmarks)
TARGETS_TEST_CC :=
TARGETS_TEST_CC += mimc

# C Targets which do not have tests
TARGETS_NOTEST_CC :=

# C Targets which are not executables
TARGETS_NOEXE_CC :=

# LIBRARY TARGET
LIBNAME := libmimc

# Put all targets together
TARGETS_CXX := $(TARGETS_ONLYTEST_CXX) $(TARGETS_TEST_CXX) $(TARGETS_NOTEST_CXX) $(TARGETS_NOEXE_CXX)
TARGETS_CC := $(TARGETS_ONLYTEST_CC) $(TARGETS_TEST_CC) $(TARGETS_NOTEST_CC) $(TARGETS_NOEXE_CC)
TARGETS_TEST := $(TARGETS_TEST_CXX) $(TARGETS_TEST_CC)
TARGETS_NOTEST := $(TARGETS_NOTEST_CXX) $(TARGETS_NOTEST_CC)
TARGETS_ONLYTEST := $(TARGETS_ONLYTEST_CXX) $(TARGETS_ONLYTEST_CC)
TARGETS_NOEXE := $(TARGETS_NOEXE_CXX) $(TARGETS_NOEXE_CC)
TARGETS := $(TARGETS_CXX) $(TARGETS_CC)
#### END TARGETS ####


#### BEGIN DIRECTORIES SETUP ####
# Directory for binaries
BINPATH := bin
# Directory for object files
BUILDPATH := build
# Directory for include files
INCPATH := include
# Directory for library files
LIBPATH := lib
# Directory for source files
SRCPATH := src
# Directory for test files
TESTPATH := test
# Prefix for test binaries
TEST_PRE := test_
# Extension path for C++ files
PATHEXT_CXX := cxx
# Extension path for C files
PATHEXT_CC := cc

# Include files
INC := $(wildcard $(INCPATH)/*.h $(INCPATH)/**/*.h $(INCPATH)/*.hpp $(INCPATH)/**/*.hpp)
# Source files
SRC := $(wildcard $(SRCPATH)/*.cpp $(SRCPATH)/**/*.cpp $(SRCPATH)/*.c $(SRCPATH)/**/*.c)
# Test files
TEST := $(wildcard $(TESTPATH)/*.cpp $(TESTPATH)/**/*.cpp $(TESTPATH)/*.c $(TESTPATH)/**/*.c)

LIBNAMES := $(TARGETS_TEST) $(TARGETS_NOTEST)
SRCNAMES := $(notdir $(SRC))
TESTNAMES := $(notdir $(TEST))
#### END DIRECTORIES SETUP ####

#### BEGIN COMPILER/LINKER SETUP ####
# If there is no default compiler, or if we want to override it, set the compiler here
ifeq ($(CC), )
    CC := cc
    CFLAGS := 
endif
ifeq ($(CXX), )
    CXX := c++
    CXXFLAGS := 
endif
ifeq ($(OVERRIDE_DEFAULT_CC), 1)
    ifeq ($(OS), Windows_NT)
        CC := cl
    else
        CC := icx
    endif
endif
ifeq ($(OVERRIDE_DEFAULT_CXX), 1)
    ifeq ($(OS), Windows_NT)
        CXX := cl
    else
        CXX := icpx
    endif
endif

# Override flags for debugging
ifneq ($(DEBUG), 0)
    ifeq ($(CC), cl)
        CFLAGS := -DEBUG -Zi
        CXXFLAGS := -DEBUG -Zi
    else
        CFLAGS := -g
        CXXFLAGS := -g
    endif
endif
ifneq ($(DEBUG), 1)
    CFLAGS += -O2
    CXXFLAGS += -O2
endif

# Additional compiler flags 
ifeq ($(CC), cl)
    CFLAGS += -nologo -arch:AVX2 -std:clatest -Wall -I$(INCPATH) -MP
    CXXFLAGS += -nologo -EHsc -arch:AVX2 -std:clatest -Wall -I$(INCPATH) -MP
    OBJOUT_FLAG := -Fo:
    EXEOUT_FLAG := -Fe:
else
    CFLAGS += -march=native -std=c++20 -Wall -I$(INCPATH) -MMD -MP
    CXXFLAGS += -march=native -std=c++20 -Wall -I$(INCPATH) -MMD -MP
    OBJOUT_FLAG := -o
    EXEOUT_FLAG := -o
endif
#### END COMPILER/LINKER SETUP ####

#### BEGIN FILES SETUP ####
# Executable file extenstion
EXEEXT := exe
# Archive extension
LIBEXT := a

# Object file extenstion
ifeq ($(CC), cl)
    OBJEXT := obj
else
    OBJEXT := o
endif

# C++ object files build path
BUILDPATH_CXX := $(BUILDPATH)/$(PATHEXT_CXX)
BUILDPATH_CC := $(BUILDPATH)/$(PATHEXT_CC)

# Object files
OBJ := $(SRCNAMES:%.cpp=$(BUILDPATH_CXX)/%.$(OBJEXT)) $(TESTNAMES:%.cpp=$(BUILDPATH_CXX)/$(TEST_PRE)%.$(OBJEXT))
OBJ += $(SRCNAMES:%.c=$(BUILDPATH_CC)%.$(OBJEXT)) $(TESTNAMES:%.cpp=$(BUILDPATH_CC)/$(TEST_PRE)%.$(OBJEXT))
# Library files
#LIB := $(LIBNAMES:%=$(BUILDPATH)/%.$(OBJEXT))
# Dependency files (allows recompiling when a header changes without `make clean`)
DEP := $(OBJ:%.$(OBJEXT)=%.d) 

# Command to run all the tests at once, will be printed to a bash script
TEST_COMMAND := $(TARGETS_TEST:%=$$SCRIPTPATH/$(TEST_PRE)%;\n)$(TARGETS_ONLYTEST:%=$$SCRIPTPATH/$(TEST_PRE)%;\n)
#### END FILES SETUP ####

#### BEGIN MACRO DEFINITION ARGUMENTS ####
# Enable performance measurament code in test files
ifeq ($(MEASURE_PERFORMANCE), 1)
    CFLAGS += -DMEASURE_PERFORMANCE
    CXXFLAGS += -DMEASURE_PERFORMANCE
endif

# Enable parallel code
ifeq ($(MULTICORE), 1)
    CFLAGS += -DMULTICORE
    CXXFLAGS += -DMULTICORE
endif

# Enable assembly code
ifeq ($(USE_ASM), 1)
    CFLAGS += -DUSE_ASM
    CXXFLAGS += -DUSE_ASM
endif

# Additional macro definitions
CFLAGS += 
CXXFLAGS += 
#### END MACRO DEFINITION ARGUMENTS ####

# Linker flags
LDFLAGS :=

# Archive command
AR := ar
# Archive flags
ARFLAGS := r

# Debug file extension (for Windows only)
PDB :=
#### END COMPILER/LINKER SETUP ####


#### BEGIN COMMAND ALIASING ####
# Make useful commands OS independent
ifeq ($(OS), Windows_NT)
    MKDIR = if not exist $(subst /,\,$1) mkdir $(subst /,\, $1)
    RM = if exist $(1) rmdir /S /Q $(1)
    MV = mv $(1) $(2)
else
    MKDIR = mkdir -p $(1)
    RM = rm -rf $(1)/*
    MV = mv $(1) $(2)/
endif
#### END COMMAND ALIASING ####

#### BEGIN MAIN RECIPES ####
# Build all targets
all: dirs testsuite $(TARGETS) library

# Remove all build files
clean:
	$(call RM,$(BUILDPATH))
	$(call RM,$(BINPATH))
	$(call RM,$(LIBPATH))

# Create build directories
dirs:
	$(call MKDIR, $(BUILDPATH))
	$(call MKDIR, $(BUILDPATH_CC))
	$(call MKDIR, $(BUILDPATH_CXX))
	$(call MKDIR,$(BINPATH))
	$(call MKDIR,$(LIBPATH))

# Generate "run all tests" script
testsuite:
	@echo -e '#!/bin/sh\nSCRIPTPATH=$$(dirname "$$(readlink -f "$$0")")\n$(TEST_COMMAND)' > $(BINPATH)/test.sh

# Build library
library: $(TARGETS)
	$(AR) $(ARFLAGS) $(LIBOUTFLAGS)$(LIBPATH)/$(LIBNAME).$(LIBEXT) $(LIBS)
#### END MAIN RECIPES ####

#### BEGIN SPECIFIC TARGET DEPENDENCIES ####
#### END SPECIFIC TARGET DEPENDENCIES ####

#### BEGIN GENERIC TARGET RECIPES ####
# Build C++ targets which only have test files
$(TARGETS_ONLYTEST_CXX): %: $(BUILDPATH_CXX)/$(TEST_PRE)%.$(OBJEXT)
	$(CXX) $(CXXFLAGS) $^ $(EXEOUT_FLAG) $(BINPATH)/$(TEST_PRE)$@.$(EXEEXT) $(LDFLAGS)

# Build C++ targets which do not have test files
$(TARGETS_NOTEST_CXX):  %: $(BUILDPATH_CXX)/%.$(OBJEXT)
	$(CXX) $(CXXFLAGS) $^ $(EXEOUT_FLAG) $(BINPATH)/$@.$(EXEEXT) $(LDFLAGS)

# Build C++ targets which have both test and source files
$(TARGETS_TEST_CXX): %: $(BUILDPATH_CXX)/$(TEST_PRE)%.$(OBJEXT) $(BUILDPATH_CXX)/%.$(OBJEXT)
	$(CXX) $(CXXFLAGS) $^ $(EXEOUT_FLAG) $(BINPATH)/$(TEST_PRE)$@.$(EXEEXT) $(LDFLAGS)

# Build C++ targets which do not have executables
$(TARGETS_NOEXE_CXX):  %: $(BUILDPATH_CXX)/%.$(OBJEXT)

# Build C targets which only have test files
$(TARGETS_ONLYTEST_CC): %: $(BUILDPATH_CC)/$(TEST_PRE)%.$(OBJEXT)
	$(CC) $(CFLAGS) $^ $(EXEOUT_FLAG) $(BINPATH)/$(TEST_PRE)$@.$(EXEEXT) $(LDFLAGS)

# Build C targets which do not have test files
$(TARGETS_NOTEST_CC):  %: $(BUILDPATH_CC)/%.$(OBJEXT)
	$(CC) $(CFLAGS) $^ $(EXEOUT_FLAG) $(BINPATH)/$@.$(EXEEXT) $(LDFLAGS)

# Build C targets which have both test and source files
$(TARGETS_TEST_CC): %: $(BUILDPATH_CC)/$(TEST_PRE)%.$(OBJEXT) $(BUILDPATH_CC)/%.$(OBJEXT)
	$(CC) $(CFLAGS) $^ $(EXEOUT_FLAG) $(BINPATH)/$(TEST_PRE)$@.$(EXEEXT) $(LDFLAGS)

# Build C targets which do not have executables
$(TARGETS_NOEXE_CC):  %: $(BUILDPATH_CC)/%.$(OBJEXT)
	

#### END GENERIC TARGET RECIPES ####

#### BEGIN GENERIC SOURCE RECIPES ####
# Compile C++ tests (main directory)
$(BUILDPATH_CXX)/$(TEST_PRE)%.$(OBJEXT): $(TESTPATH)/%.cpp
	$(CXX) $(CXXFLAGS) -c $< $(OBJOUT_FLAG) $@

# Compile C++ tests (recursive)
$(BUILDPATH_CXX)/$(TEST_PRE)%.$(OBJEXT): $(TESTPATH)/**/%.cpp
	$(CXX) $(CXXFLAGS) -c $< $(OBJOUT_FLAG) $@

# Compile C++ sources (main directory)
$(BUILDPATH_CXX)/%.$(OBJEXT): $(SRCPATH)/%.cpp
	$(CXX) $(CXXFLAGS) -c $< $(OBJOUT_FLAG) $@

# Compile C++ sources (recursive)
$(BUILDPATH_CXX)/%.$(OBJEXT): $(SRCPATH)/**/%.cpp
	$(CXX) $(CXXFLAGS) -c $< $(OBJOUT_FLAG) $@

# Compile C tests (main directory)
$(BUILDPATH_CC)/$(TEST_PRE)%.$(OBJEXT): $(TESTPATH)/%.c
	$(CC) $(CFLAGS) -c $< $(OBJOUT_FLAG) $@

# Compile C tests (recursive)
$(BUILDPATH_CC)/$(TEST_PRE)%.$(OBJEXT): $(TESTPATH)/**/%.c
	$(CC) $(CFLAGS) -c $< $(OBJOUT_FLAG) $@

# Compile C sources (main directory)
$(BUILDPATH_CC)/%.$(OBJEXT): $(SRCPATH)/%.c
	$(CC) $(CFLAGS) -c $< $(OBJOUT_FLAG) $@

# Compile C sources (recursive)
$(BUILDPATH_CC)/%.$(OBJEXT): $(SRCPATH)/**/%.c
	$(CC) $(CFLAGS) -c $< $(OBJOUT_FLAG) $@
#### END GENERIC SOURCE RECIPES ####
