import sys
import re

contains_dup = re.compile(r"(..).*\1");
contains_axa = re.compile(r"(.).\1");
num_matches = 0

for l in sys.stdin.readlines():
    w = l.strip()
    if contains_dup.search(w) and contains_axa.search(w):
        num_matches = num_matches + 1

print "Num matches %d" % num_matches


