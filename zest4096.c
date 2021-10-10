#include <stdint.h>

// Rotate Left
#define ROTL(a, b) (((a) << (b)) | ((a) >> (64 - (b))))
// Perform an eighth of a round
#define RE(a, b, c, d, e, f, g, h) (	\
a += b, h = ROTL(h ^ a, 32), 			\
c += d, b = ROTL(b ^ c, 28), 			\
e += f, d = ROTL(d ^ e, 24), 			\
g += h, f = ROTL(f ^ g, 21),			\
a += b, h = ROTL(h ^ a, 16), 			\
c += d, b = ROTL(b ^ c, 12), 			\
e += f, d = ROTL(d ^ e,  8), 			\
g += h, f = ROTL(f ^ g,  5))

// Input and Output block size of 64 * 64, or 4096 bits (512 bytes)
void zest4096(uint64_t out[64], const uint64_t in[64]) {
	// Copy input to working value array
	uint64_t x[64];
	for (int i = 0; i < 64; ++i)
		x[i] = in[i];
	// Perform 48 * 2 rounds
	for (uint32_t i = 0; i < 48; i++) {
		// Odd round (columns)
		RE(x[0], x[ 8], x[16], x[24], x[32], x[40], x[48], x[56]);
		RE(x[1], x[ 9], x[17], x[25], x[33], x[41], x[49], x[57]);
		RE(x[2], x[10], x[18], x[26], x[34], x[42], x[50], x[58]);
		RE(x[3], x[11], x[19], x[27], x[35], x[43], x[51], x[59]);
		RE(x[4], x[12], x[20], x[28], x[36], x[44], x[52], x[60]);
		RE(x[5], x[13], x[21], x[29], x[37], x[45], x[53], x[61]);
		RE(x[6], x[14], x[22], x[30], x[38], x[46], x[54], x[62]);
		RE(x[7], x[15], x[23], x[31], x[39], x[47], x[55], x[63]);
		// Even round (diagonals)
		RE(x[0], x[ 9], x[18], x[27], x[36], x[45], x[54], x[63]);
		RE(x[1], x[10], x[19], x[28], x[37], x[46], x[55], x[56]);
		RE(x[2], x[11], x[20], x[29], x[38], x[47], x[48], x[57]);
		RE(x[3], x[12], x[21], x[30], x[39], x[40], x[49], x[58]);
		RE(x[4], x[13], x[22], x[31], x[32], x[41], x[50], x[59]);
		RE(x[5], x[14], x[23], x[24], x[33], x[42], x[51], x[60]);
		RE(x[6], x[15], x[16], x[25], x[34], x[43], x[52], x[61]);
		RE(x[7], x[ 8], x[17], x[26], x[35], x[44], x[53], x[62]);
	}
	// Add to original input
	for (int i = 0; i < 64; ++i)
		out[i] = x[i] + in[i];
}
