SRC    = $(wildcard src/*.c)
CC     = gcc
CFLAGS = -Wall -Wextra
LIBS   = -lm
OBJ    = $(SRC:src/%.c=build/%.o)

all: clean build detector

detector: ${OBJ}
	$(CC) $(LIBS) -o $@ $^

build:
	mkdir $@

build/%.o: src/%.c
	$(CC) $(CFLAGS) -c -o $@ $^

clean:
	rm -rf detector *.exe
