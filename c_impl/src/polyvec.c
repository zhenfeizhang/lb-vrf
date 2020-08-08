#include <stdint.h>
#include "param.h"
#include "poly.h"
#include "polyvec.h"
/*************************************************
* Name:        polyvecl_add
*
* Description: Add vectors of polynomials of length L.
*              No modular reduction is performed.
*
* Arguments:   - polyvecl *w: pointer to output vector
*              - const polyvecl *u: pointer to first summand
*              - const polyvecl *v: pointer to second summand
**************************************************/
void polyvec4_add(polyvec4 *w, const polyvec4 *u, const polyvec4 *v) {
  unsigned int i;

  for(i = 0; i < 4; ++i)
    poly_add(&w->vec[i], &u->vec[i], &v->vec[i]);
}


/*************************************************
* Name:        polyvecl_chknorm
*
* Description: Check infinity norm of polynomials in vector of length L.
*              Assumes input coefficients to be standard representatives.
*
* Arguments:   - const polyvecl *v: pointer to vector
*              - uint32_t B: norm bound
*
* Returns 0 if norm of all polynomials is strictly smaller than B and 1
* otherwise.
**************************************************/
int polyvec4_chknorm(const polyvec4 *v, uint32_t bound)  {
  unsigned int i;

  for(i = 0; i < L; ++i)
    if(poly_chknorm(&v->vec[i], bound))
      return 1;

  return 0;
}

/*************************************************
* Name:        polyveck_csubq
*
* Description: For all coefficients of polynomials in vector of length K
*              subtract Q if coefficient is bigger than Q.
*
* Arguments:   - polyveck *v: pointer to input/output vector
**************************************************/
void polyvec4_csubq(polyvec4 *v) {
  unsigned int i;

  for(i = 0; i < 4; ++i)
    poly_csubq(&v->vec[i]);
}

/*************************************************
* Name:        polyveck_add
*
* Description: Add vectors of polynomials of length K.
*              No modular reduction is performed.
*
* Arguments:   - polyveck *w: pointer to output vector
*              - const polyveck *u: pointer to first summand
*              - const polyveck *v: pointer to second summand
**************************************************/
void polyvec9_add(polyvec9 *w, const polyvec9 *u, const polyvec9 *v) {
  unsigned int i;

  for(i = 0; i < 9; ++i)
    poly_add(&w->vec[i], &u->vec[i], &v->vec[i]);
}


/*************************************************
* Name:        polyveck_sub
*
* Description: Subtract vectors of polynomials of length K.
*              Assumes coefficients of polynomials in second input vector
*              to be less than 2*Q. No modular reduction is performed.
*
* Arguments:   - polyveck *w: pointer to output vector
*              - const polyveck *u: pointer to first input vector
*              - const polyveck *v: pointer to second input vector to be
*                                   subtracted from first input vector
**************************************************/
void polyvec9_sub(polyvec9 *w, const polyvec9 *u, const polyvec9 *v) {
  unsigned int i;

  for(i = 0; i < 9; ++i)
    poly_sub(&w->vec[i], &u->vec[i], &v->vec[i]);
}


/*************************************************
* Name:        polyveck_chknorm
*
* Description: Check infinity norm of polynomials in vector of length K.
*              Assumes input coefficients to be standard representatives.
*
* Arguments:   - const polyveck *v: pointer to vector
*              - uint32_t B: norm bound
*
* Returns 0 if norm of all polynomials are strictly smaller than B and 1
* otherwise.
**************************************************/
int polyvec9_chknorm(const polyvec9 *v, uint32_t bound) {
  unsigned int i;

  for(i = 0; i < 9; ++i)
    if(poly_chknorm(&v->vec[i], bound))
      return 1;

  return 0;
}
