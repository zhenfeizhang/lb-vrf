#ifndef POLY_H
#define POLY_H

#include <stdint.h>
#include "param.h"

typedef struct {
  uint32_t coeffs[D];
} poly __attribute__((aligned(32)));

void poly_csubq(poly *a);

void poly_add(poly *c, const poly *a, const poly *b);
void poly_sub(poly *c, const poly *a, const poly *b);

//void poly_mul_index(poly *c, const poly *a, const poly *index);

int  poly_chknorm(const poly *a, uint32_t B);
void poly_uniform(poly *a,
                  const unsigned char seed[SEEDBYTES],
                  uint16_t nonce);
void poly_uniform_eta(poly *a,
                  const unsigned char seed[SEEDBYTES],
                  uint16_t nonce);
void poly_uniform_beta_m_one(poly *a,
                  const unsigned char seed[CRHBYTES],
                  uint16_t nonce);


void hexDump (const char *desc, const void *addr, const int len);
#endif
