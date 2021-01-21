import math
test = int(input())
_f = []
for t in range(test):
    c1 = input()
    [L1, R1] = c1.split(" ")
    L = int(L1)
    R = int(R1)
    for i in range(L, R+1):
        _t = math.factorial(i-1)
        _b = math.factorial(2)
        _d = int(_t/_b)
        if _d%i == 0:
            _f.append(i)
print(len(_f))