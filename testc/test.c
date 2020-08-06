#include <sodium.h>
#include <stdio.h>
#include <assert.h>
// Credit: https://stackoverflow.com/questions/7775991/how-to-get-hexdump-of-a-structure-data
void hexDump (const char *desc, const void *addr, const int len);


int main(){
  if (sodium_init() < 0) {
    /* panic! the library couldn't be initialized, it is not safe to use */
  }
  unsigned char pk[crypto_vrf_PUBLICKEYBYTES];
  unsigned char sk[crypto_vrf_SECRETKEYBYTES];
  unsigned char proof[crypto_vrf_PROOFBYTES];
  unsigned char output[crypto_vrf_OUTPUTBYTES];


  int rt = crypto_vrf_keypair(pk, sk);

  hexDump("pk", pk, crypto_vrf_PUBLICKEYBYTES);
  hexDump("sk", sk, crypto_vrf_SECRETKEYBYTES);

  const unsigned char m[28] = "this is the message to sign";
  int mlen = 28;
  rt += crypto_vrf_prove(proof, sk, m, mlen);
  hexDump("proof", proof, crypto_vrf_PROOFBYTES);

  assert(crypto_vrf_verify(output, pk, proof, m, mlen)==0);
  hexDump("output", output, crypto_vrf_OUTPUTBYTES);

  printf("we are done!\n");
  return rt;
}




// Credit: https://stackoverflow.com/questions/7775991/how-to-get-hexdump-of-a-structure-data
void hexDump (const char *desc, const void *addr, const int len) {
    int i;
    unsigned char buff[17];
    const unsigned char *pc = (const unsigned char*)addr;

    // Output description if given.
    if (desc != NULL)
        printf ("%s:\n", desc);

    if (len == 0) {
        printf("  ZERO LENGTH\n");
        return;
    }
    if (len < 0) {
        printf("  NEGATIVE LENGTH: %i\n",len);
        return;
    }

    // Process every byte in the data.
    for (i = 0; i < len; i++) {
        // Multiple of 16 means new line (with line offset).

        if ((i % 16) == 0) {
            // Just don't print ASCII for the zeroth line.
            if (i != 0)
                printf ("  %s\n", buff);

            // Output the offset.
            printf ("  %04x ", i);
        }

        // Now the hex code for the specific character.
        printf (" %02x", pc[i]);

        // And store a printable ASCII character for later.
        if ((pc[i] < 0x20) || (pc[i] > 0x7e))
            buff[i % 16] = '.';
        else
            buff[i % 16] = pc[i];
        buff[(i % 16) + 1] = '\0';
    }

    // Pad out last line if not exactly 16 characters.
    while ((i % 16) != 0) {
        printf ("   ");
        i++;
    }

    // And print the final ASCII bit.
    printf ("  %s\n", buff);
}
