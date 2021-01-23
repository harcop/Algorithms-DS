# a = input()
a1 = input().split(" ")
# c = input()
c1 = input().split(" ")
print(a1, c1)
_f = []
for i in a1:
    for j in c1:
        v = abs(int(j)-int(i))
        if str(v) not in _f:
            if str(v) in c1 or str(v) in a1:
                _f.append(str(v))
_f.sort()
print(" ".join(_f))