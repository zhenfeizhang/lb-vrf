#ifndef LBVRF_PARAM_H
#define LBVRF_PARAM_H

#include <stdlib.h>


/// P is the modulus for `B part`
#define P 2097169



/// Q is the modulus for `A part`
#define Q 100679681
#define MINUS_ONE 100679680
///// SAMPLE_Q = 2^32 - (2^32//Q)*Q
//#define SAMPLE_Q  4228546602

/// R is a root s.t. (x^32+R) divides (x^256+1) mod P
#define R 852368

/// degree of the ring
#define D 256

/// rank of MLWE
#define L 4

/// rank of SIS
#define N 4

/// hamming weight of challenge
#define C 39

#define BETA 89856

/// threshold \beta - kappa * alpha
#define THRESHOLD 89817

#define SECURITY_LEVEL 128
#define SEEDBYTES 32
#define CRHBYTES 32
#define PUBPARAM_LEN 32
#define PUBKEY_LEN 32
#define SECKEY_LEN 32
#define VRFPROOF_LEN 32
#define VRFOUTPUT_LEN 32

#define A_ROW 4
#define A_COL 9
#define B_ROW 1
#define B_COL 9
#define RING_SIZE 256
#define CRT_RING_SIZE 32


typedef	uint32_t ring_element[RING_SIZE];
typedef uint32_t crt_ring_element[CRT_RING_SIZE];

// input: a seed of SEEDLEN
// output: a public parameter MATRIX A




#endif
