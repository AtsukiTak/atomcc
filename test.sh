#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  target/debug/atomcc "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert_fn() {
  expected="$1"
  input="$2"
  link="$3"

  target/debug/atomcc "$input" > tmp.s
  cc -o tmp "$link" tmp.s
  output=$(./tmp)

  if [ "$output" = "$expected" ]; then
    echo "$input => $output"
  else
    echo "$input => $expected expected, but got $output"
    exit 1
  fi
}

cargo build

assert 0 "0;"
assert 42 "42;"
assert 56 "40+16;"
assert 78 "100-22;"
assert 121 " 42 + 90 - 11; "
assert 12 " -3 + 5 * 3;"
assert 47 '5+6*7;'
assert 15 '5*(9-6);'
assert 4 '(3+5)/2;'
assert 4 '+4;'
assert 2 '-14+16;'
assert 20 '-(4+6)*2+40;'
assert 0 '1 == 0;'
assert 1 '1 != 0;'
assert 1 '(1 + 40) > 2 * 10;'
assert 1 '(1 + 40) >= 2 * 10 + 21;'
assert 24 '42;24;'
assert 42 'a = 40; a + 2;'
assert 42 'foo = 40; foo + 2;'
assert 42 'return 42;'
assert 42 'foo = 11; return foo + 31;'
assert 22 'foo = 11; if ( foo == 11 ) 22; else 42;'
assert 42 'foo = 11; if(foo == 10) 22; else 42;'
assert 10 'i = 1; while (i<10) i = i+1; return i;'
assert 45 '
i = 1;
n = 0;
while (i < 10) {
  n = n + i;
  i = i + 1;
}
return n;
'

assert_fn OK 'foo();' tests/foo.c
assert_fn 42 '42; foo2(10, 32);' tests/foo.c

echo OK
