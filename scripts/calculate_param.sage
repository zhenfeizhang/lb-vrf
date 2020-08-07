## this script find the following parameters:
## q = 100679681 = 0x0x6004001; hamming(q) = 4
## p = 2097169 = 0x200011; hamming(p) = 3
## r = 852368 = 0xd0190; hamming(r) = 6
## (x^32 + r) is a factor of (x^256+1) mod p



min_q = ZZ(round(2^26.33))
min_p = ZZ(2^20)
d = 256



def hamming(a):
    r = 0
    for e in a.bits():
        r += e
    return r


# find q s.t.
# 1. q%2d == 1
# 2. q is prime
# 3. hamming(q) is minimal
# 4. q < 2^27

print "finding q"
q = next_prime(min_q)
res = q
hm  = 4     ## we have a valid q = 0x6004001, hm = 4

while (q<2^27):
    q = next_prime(q)
    if hamming(q)<=4 and q%(2*d) == 1:
        print q, hex(q), hamming(q)


# three candidate q
# 100679681 0x6004001 4
# 101711873 0x6100001 4
# 104857601 0x6400001 4

def best_root(p):

    P.<x> = PolynomialRing(Zmod(p))
    f = P(x^256+1)
    t = f.factor()
    res = t[0][0][0]

    hm = hamming(ZZ(res))
    for e in t:
        root = ZZ(e[0][0])
        # print root, hamming(root)
        if hamming(root) < hm:
            hm = hamming(root)
            res = root
#    print root, hamming
    return res



# find p s.t.
# 1. p%2d == 1
# 2. p is prime
# 3. hamming(p) is minimal
# 4. p < 2^27
# 5. (x^256+1 mod q) has

print "finding q"
p = next_prime(min_p)
res = p
hm  = 3   ## we have a valid p = 0x200011
while (p<2^21):
    p = next_prime(p)
    if hamming(p)<=hm and p%32 == 17:
        print p, hex(p), hamming(p), best_root(p)
