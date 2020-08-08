/*
 ============================================================================
 Name        : c_impl.c
 Author      : Zhenfei Zhang
 Version     :
 Copyright   : MIT
 Description : Hello World in C, Ansi-style
 ============================================================================
 */

#include <stdio.h>
#include <stdlib.h>
#include "param.h"
#include "lbvrf.h"
#include "imported/misc.h"
#include "polyvec.h"
//#include "imported/randombytes.h"

int main(void) {

	unsigned char a[32];

//	randombytes(a, 32);

	const uint8_t* seed = "this is a seed for testing. do not use this seed in product!";
	const size_t seed_len = strlen(seed);
	char str[64];

	ring_element** matrix_A = lbvrf_pubparam(seed, seed_len);

	int i,j,k;
//	for(i=0;i<A_ROW;i++){
//		for (j=0;j<A_COL;j++){
//			sprintf(str, "A[%d][%d]", i, j);
//			hexDump(str, matrix_A[i][j], RING_SIZE*sizeof(uint32_t));
//		}
//	}
	ring_element* vector_s = malloc(sizeof(ring_element)*A_COL);
	ring_element* vector_b = malloc(sizeof(ring_element)*A_ROW);

	lbvrf_keygen(matrix_A, vector_b, vector_s);

	for(i=0;i<A_COL;i++){
		sprintf(str, "S[%d]", i);
		hexDump(str, vector_s[i], RING_SIZE*sizeof(uint32_t));
	}

//	lbvrf_keygen(matrix_A, vector_b, vector_b);
	ring_element* t;

//	t = poly_mul_trinary(vector_s[0], vector_s[1]);
	printf("?\n");
//	hexDump("prod", t[0], RING_SIZE*sizeof(uint32_t));

	polyvec9 mat_A[4];
	expand_mat(mat_A, seed);

	for(i=0;i<4;i++){
		for (j=0;j<9;j++){
			sprintf(str, "A[%d][%d]", i, j);
			hexDump(str, mat_A[i].vec[j].coeffs, RING_SIZE*sizeof(uint32_t));
		}
	}
	puts("!!!Hello Algorand!!!"); /* prints !!!Hello Algorand!!! */
	return EXIT_SUCCESS;
}
