load Or4Way16.hdl,
output-file Or4Way16.out,
compare-to Or4Way16.cmp,
output-list a%B1.16.1 b%B1.16.1 c%B1.16.1 d%B1.16.1 out%B1.16.1;

set a 0,
set b 0,
set c 0,
set d 0,
eval,
output;

set a %B1000000000000000,
set b 0,
set c 0,
set d 0,
eval,
output;

set a %B1000000000000000,
set b %B0100000000000000,
set c %B0010000000000000,
set d %B0001000000000000,
eval,
output;

set a %B1000000000000000,
set b %B1100000000000000,
set c %B0010000000000000,
set d %B0001000000000000,
eval,
output;

set a %B1111111111111111,
set b %B1111111111111111,
set c %B1111111111111111,
set d %B1111111111111111,
eval,
output;
