test = int(input())
ba = 0
for t in range(test):
    c1 = input()
    [N1, P1] = c1.split(" ")
    N = int(N1)
    P = int(P1)
    milk = input().split(" ")
    apple = input().split(" ")
    P -= 1
    for i in range(N):
        if P == 0:
            P += int(milk[i])
        else:
            ba += int(apple[i])
            P -= 1
print(ba)