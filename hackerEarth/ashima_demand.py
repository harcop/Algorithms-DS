demand = input()
test = int(input())
_f = []
for i in range(test):
    c1 = input()
    [a1, b1] = c1.split(" ")
    a = int(a1)
    b = int(b1)
    if b%a == 0:
        _f.append("YES")
    else:
        _f.append("NO")
for i in _f:
    print(i)