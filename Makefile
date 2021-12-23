SRC    := $(wildcard src/*.c)
CC     := gcc
CFLAGS := -Wall -Wextra -fdata-sections -ffunction-sections
LFLAGS := -Wl,--gc-sections -Wl,-s
LIBS   := -lm
OBJ    := $(SRC:src/%.c=build/%.o)

all: clean build detector

detector: ${OBJ}
	$(CC) $(LIBS) $(LFLAGS) -o $@ $^

build:
	mkdir $@

build/%.o: src/%.c
	$(CC) $(CFLAGS) -c -o $@ $^

clean:
	rm -rf build detector *.exe
