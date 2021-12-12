#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <unistd.h>

#include "skin.h"

#define STB_IMAGE_IMPLEMENTATION
#define STB_IMAGE_STATIC
#define STBI_NO_LINEAR
#define STBI_NO_HDR
#define STBI_NO_PNG
#define STBI_NO_BMP
#define STBI_NO_PSD
#define STBI_NO_TGA
#define STBI_NO_GIF
#define STBI_NO_HDR
#define STBI_NO_PIC
#define STBI_NO_PNM
#include "../stb/stb_image.h"

#define STB_IMAGE_WRITE_IMPLEMENTATION
#define STB_IMAGE_WRITE_STATIC
#include "../stb/stb_image_write.h"

static void usage(const char *self) {
	printf("Usage: %s <input> <output>\n", self);
	exit(1);
}

int main(int argc, const char *argv[]) {
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

	// Checking each individual pixel for skin in it
	for (int32_t x = 0; x < width; x+=channels) {
		for (int32_t y = 0; y < height; y+=channels) {
			const int32_t current = x + width * y;

			// Getting pixel offset
			const uint8_t *p = image + current * channels;

			// Color equals 255 if pixel is a skin or 0 if pixel isn't
			const uint8_t color = skin(p[0], p[1], p[2]) ? 255 : 0;

			// Setting color result for each channel
			for (int32_t c = 0; c < channels; c++)
				image[current + c] = color;
		}
	}

	const char* output = argv[2];
	stbi_write_png(output, width, height, channels, image, width * channels);
	free(image);
	return 0;
}
