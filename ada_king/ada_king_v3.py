import sys

t = int(input())
for tt in range(t):
    k = int(input())
    ans = [['.' for i in range(8)] for j in range(8)]
    ans[7][7] = 'O'
    c = 64 - k
    for i in range(8):
        for j in range(8):
            if c > 0:
                ans[i][j] = 'X'
                c -= 1
    for i in range(8):
        print("".join(ans[i]))