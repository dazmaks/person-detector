#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <unistd.h>


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
	const char* output = argv[2];
	int32_t width, height, channels;
	uint8_t* image = stbi_load(input, &width, &height, &channels, 0);

	if(image == NULL) {
		perror(input);
		exit(1);
	}

	char* outputimg = malloc(2147483647);
	for (int32_t x = 0; x < width; x++) {
		for (int32_t y = 0; y < height; y++) {
			int32_t current = x + width * y;
			const uint8_t *p = image + channels * current;
			const uint8_t color = skin(p[0], p[1], p[2]) ? 255 : 0;
			outputimg[current] = color;
		}
	}

	stbi_write_png(output, width, height, channels, outputimg, width * channels);
	stbi_image_free(image);
	return 0;
}
