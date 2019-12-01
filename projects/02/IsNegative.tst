load IsNegative.hdl,
output-file IsNegative.out,
compare-to IsNegative.cmp,
output-list in%B1.16.1 out%B3.1.3;

set in %B0000000000000000,
eval,
output;

set in %B0111111111111111,
eval,
output;

set in %B1000000000000000,
eval,
output;

set in %B1111111111111111,
eval,
output;
