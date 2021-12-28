#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>

#define STB_IMAGE_IMPLEMENTATION
#include "../stb/stb_image.h"

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "../stb/stb_image_write.h"

#include "skin.h"

static void usage(const char *self) {
	printf("Usage: %s <input> <output>\n", self);
	exit(1);
}

int main(int argc, char const *argv[]) {
	if(argc != 3) {
		usage(argv[0]);
	}
	const char* input = argv[1];
	int32_t width, height, channels;
	uint8_t* image = stbi_load(input, &width, &height, &channels, 0);

	if(image == NULL) {
		perror(input);
		exit(1);
	}

	// Checking each individual pixel for skin with our filter
	int32_t x, y, c;
	for (x = 0; x < width; x+=3) {
		for (y = 0; y < height; y+=3) {
			// Current pixel equals x + width * y
			const int32_t current = x + width * y;

			// Getting pixel offset
			const uint8_t *p = image + current * channels;

			// Getting pixel color (255 if it is skin or 0 if it isn't)
			const uint8_t color = skin(p[0], p[1], p[2]) ? 255 : 0;

			// Setting current pixel for all channels
			for (c = 0; c < channels; image[current + c++] = color);
		}
	}

	const char* output = argv[2];
	stbi_write_png(output, width, height, channels, image, width * channels);
	free(image);
	return 0;
}
