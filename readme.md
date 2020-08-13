lattice based verifiable random function
-----

- [x] one time LB-VRF scheme
  - [x] implement a basic scheme
  - [x] use crt to compress vrf outputs
  - [ ] implement NTT to accelerate polynomial multiplications
  - [ ] better serialization to reduce sizes
  - [ ] use Montgomery representations to further improve performance
- [ ] many time VRF scheme
  - [ ] decide which long term signature scheme we want to use
  - [ ] implement scheme
- [ ] (a lot) more tests
- [ ] zeroize memory
- [ ] constant time implementation (seems not required)
