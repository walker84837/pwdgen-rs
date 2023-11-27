# Define variables
CC = gcc
CFLAGS = -Ofast -march=native -Wall -Wextra -pedantic -std=gnu99 -pie $(shell pkg-config --cflags libssl libcrypto)
LDFLAGS = $(shell pkg-config --libs libssl libcrypto)
SRCDIR = src
OBJDIR = obj
BINDIR = bin
SOURCES = $(wildcard $(SRCDIR)/*.c)
OBJECTS = $(patsubst $(SRCDIR)/%.c, $(OBJDIR)/%.o, $(SOURCES))
EXECUTABLE = $(BINDIR)/passwordgen

# Default target
all: $(EXECUTABLE)


# Compile the source files into object files
$(OBJDIR)/%.o: $(SRCDIR)/%.c
	$(shell mkdir -p $(OBJDIR) $(BINDIR))
	$(CC) $(CFLAGS) -c $< -o $@ $(LDFLAGS)

# Link the object files to create the executable
$(EXECUTABLE): $(OBJECTS)
	$(shell mkdir -p $(OBJDIR) $(BINDIR))
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

# Clean up the generated files
clean:
	rm -rf $(OBJDIR) $(BINDIR) 'passwordgen'

# Run the program
run: $(EXECUTABLE)
	./$(EXECUTABLE)

.PHONY: all clean run
