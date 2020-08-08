#include "lbvrf.h"
#include "param.h"
#include "imported/rng.h"
// generate a pair of public and secret keys
void lbvrf_keygen(const ring_element** matrix_A, ring_element* vector_b, ring_element* vector_s){

	char rng_seed[] = "warning: this is a seed for testing\nshould only not be used for other purpose\n";
	char salt[] = "warning: this is a salt for testing\nshould only not be used for other purpose\n";
	char key_seed[SEEDBYTES];

	printf("%s", rng_seed);

//	randombytes_init(rng_seed, salt, SECURITY_LEVEL);
//	randombytes(key_seed, SEEDBYTES);
//
//	lbvrf_keygen_from_seed(matrix_A, vector_b, vector_s, key_seed, SEEDBYTES);
}


void lbvrf_keygen_from_seed(const ring_element** matrix_A, ring_element* vector_b, ring_element* vector_s, uint8_t* seed, size_t seed_len){
//	char salt[] = "this is a public salt to generate secret keys for LBVRF scheme";
//	randombytes_init(seed, salt, SECURITY_LEVEL);
//
//
//	uint8_t tmp;
//	int ct = 0;
//	int i = 0;
//	int j = 0;
//	randombytes(&tmp, 1);
//	for (j=0;j<A_COL;j++){
//		for (i=0;i<D;i++){
////			if (tmp%3==0)
////				vector_s[j][i] = MINUS_ONE;
////			else
//				vector_s[j][i] = tmp%3-1;
//			ct++;
//			if (ct==3){
//				randombytes(&tmp, 1);
//				ct = 0;
//			}
//		}
//	}

	return;
}
