load And16Way.hdl,
output-file And16Way.out,
compare-to And16Way.cmp,
output-list in%B1.16.1 out%B3.1.3;

set in %B0000000000000000,
eval,
output;

set in %B0101010101010101,
eval,
output;

set in %B1111111111111111,
eval,
output;
