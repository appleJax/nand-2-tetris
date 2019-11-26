load Or8Way16.hdl,
output-file Or8Way16.out,
compare-to Or8Way16.cmp,
output-list a%B1.16.1 b%B1.16.1 c%B1.16.1 d%B1.16.1 e%B1.16.1 f%B1.16.1 g%B1.16.1 h%B1.16.1 out%B1.16.1;

set a 0,
set b 0,
set c 0,
set d 0,
set e 0,
set f 0,
set g 0,
set h 0,
eval,
output;

set a %B0000000000000001,
eval,
output;

set a 0,
set b 1,
eval,
output;

set b 0,
set c 1,
eval,
output;

set c 0,
set d 1,
eval,
output;

set d 0,
set e 1,
eval,
output;

set e 0,
set f 1,
eval,
output;

set f 0,
set g 1,
eval,
output;

set g 0,
set h 1,
eval,
output;

set a 1,
set b 2,
set c 4,
set d 8,
set e 16,
set f 32,
set g 64,
set h 128,
eval,
output;

set a %B1000000000000000,
set b 16384,
set c 8192,
set d 4096,
set e 2048,
set f 1024,
set g 512,
set h 256,
eval,
output;
