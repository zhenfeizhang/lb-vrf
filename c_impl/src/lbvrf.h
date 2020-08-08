#ifndef LBVRF_H
#define LBVRF_H

#include <stdlib.h>


typedef	uint32_t ring_element[256];
typedef uint32_t crt_ring_element[32];


// input a seed of seed_len
// output a public parameter matrix A
ring_element** lbvrf_pubparam(const uint8_t* seed, const size_t seed_len);


// generate a pair of public and secret keys, serialized
void lbvrf_keypair(uint8_t* pk, uint8_t* sk);


// generate a pair of public and secret keys
void lbvrf_keygen(const ring_element** matrix_A, ring_element* vector_b, ring_element* vector_s);

// generate a pair of public and secret keys
void lbvrf_keygen_from_seed(const ring_element** matrix_A, ring_element* vector_b, ring_element* vector_s, uint8_t* seed, size_t seed_len);


// generate a pair of public and secret keys from seed
void lbvrf_keypair_from_seed(uint8_t* pk, uint8_t* sk, const uint8_t* seed, const size_t seed_len);

#endif
