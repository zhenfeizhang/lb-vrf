#ifndef POLYVEC_H
#define POLYVEC_H

#include <stdint.h>
#include "param.h"
#include "poly.h"

/* Vectors of polynomials of length L */
typedef struct {
  poly vec[4];
} polyvec4;


void polyvec4_add(polyvec4 *w, const polyvec4 *u, const polyvec4 *v);


int polyvec4_chknorm(const polyvec4 *v, uint32_t B);



/* Vectors of polynomials of length  */
typedef struct {
  poly vec[9];
} polyvec9;


void polyvec9_add(polyvec9 *w, const polyvec9 *u, const polyvec9 *v);
void polyvec9_sub(polyvec9 *w, const polyvec9 *u, const polyvec9 *v);

int polyvec9_chknorm(const polyvec9 *v, uint32_t B);

#endif
