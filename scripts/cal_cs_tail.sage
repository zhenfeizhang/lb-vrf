'''
this scripts calculates the tail of c*s
* input: dimension d
* input: distribution of s: [-alpha, alpha]
* input: hamming weight of c: h
* output: the tail t
* ensure:
    Prob[ |cs|_\infty < t ] > 1 - 2^128
'''


from proba_util import *

alpha = 1
h = 39
d = 256

#chi = {-5: 1/11, -4: 1/11, -3: 1/11, -2: 1/11, -1: 1/11, 0: 1/11, 1: 1/11, 2: 1/11, 3: 1/11, 4: 1/11, 5: 1/11}


def get_bd(alpha, h, d):

    max_val = h*alpha
    base = alpha*2 +1
    chi = {0: 1/base}
    for i in range(1, alpha+1):
        chi.update({i: 1/base})
        chi.update({-i: 1/base})
    chi_h = iter_law_convolution(chi, h)

    # single_bd is the prob that a single coeff has the desired probability
    single_bd = 0
    counter = 0
    while d * single_bd < 2^-128:
        ## here we have d * single_bd to get the union bound
        single_bd += chi_h.get(max_val-counter)*2
        counter += 1

        print max_val-counter, log(single_bd,2), log(single_bd*d, 2)


get_bd(alpha, 39, d)
get_bd(alpha, 60, d)
