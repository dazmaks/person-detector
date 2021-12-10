#include <stdlib.h>
#include "skin.h"

#define min(a,b) (((a)<(b))?(a):(b))
#define max(a,b) (((a)>(b))?(a):(b))

#define min3(a,b,c) (min(min(a, b), c))
#define max3(a,b,c) (max(max(a, b), c))

int32_t skin(uint8_t r, uint8_t g, uint8_t b) {
	return r > 90 && g > 40 && r > g && r > b && abs(r-g) > 15 && max3(r, g, b) - min3(r, g, b) > 15;
}
