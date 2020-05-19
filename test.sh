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

cargo build

assert 0 0
assert 42 42
assert 56 "40+16"
assert 78 "100-22"
assert 121 " 42 + 90 - 11 "
assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'
assert 4 '+4'
assert 2 '-14+16'
assert 20 '-(4+6)*2+40'

echo OK
