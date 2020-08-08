#include "lbvrf.h"
#include "param.h"
#include "imported/rng.h"
#include "polyvec.h"
#include "poly.h"

// input a seed of seed_len
// output a public parameter matrix A
ring_element** lbvrf_pubparam(const uint8_t* seed, const size_t seed_len){

	if (seed_len<SEEDBYTES) {
		fprintf(stderr, "Seed is too short: %d\n", seed_len);
		return NULL;
	}
	unsigned char salt[] = "lattice based cryptography is awesome!!!";
	randombytes_init(seed, salt, SECURITY_LEVEL);
	unsigned char buf[4];


	ring_element** param_A = malloc(sizeof(ring_element*) * A_ROW);

	int i, j, k;
	uint32_t tmp;
	for(i=0;i<A_ROW;i++){
		param_A[i] = malloc(sizeof(ring_element) * A_COL);

		for(j=0;j<A_COL;j++){
			for(k=0;k<RING_SIZE;k++){
				// todo: revisit this optimization
				// param_A[i][j][k]< 0x8000000 which is just a little bit smaller than q = 0x800001d
				randombytes(buf, 4);

				param_A[i][j][k]= 	((buf[0]& 0b111 )<<24) + \
									(buf[1]<<16) + \
									(buf[2]<<8) + \
									buf[3];
//				printf("%x %x %dx %x %x\n", buf[0], buf[1], buf[2], buf[3], param_A[i][j][k]);
			}
		}
	}
	return param_A;
}


int get_matrix_A_size(){
	return sizeof(uint32_t) * RING_SIZE * A_ROW * A_COL;
}


/*************************************************
* Name:        expand_mat
*
* Description: Implementation of ExpandA. Generates matrix A with uniformly
*              random coefficients a_{i,j} by performing rejection
*              sampling on the output stream of SHAKE128(rho|i|j).
*
* Arguments:   - polyvecl mat[K]: output matrix
*              - const unsigned char rho[]: byte array containing seed rho
**************************************************/
void expand_mat(polyvec9 mat[4], const unsigned char rho[SEEDBYTES]) {
  unsigned int i, j;

  for(i = 0; i < 4; ++i)
    for(j = 0; j < 9; ++j)
      poly_uniform(&mat[i].vec[j], rho, (i << 8) + j);
}