tests = int(input())
for i in range(tests):
    gr = input()
    c1 = input()
    c2 = input()
    p = int(input())
    [n, m] = gr.split(" ")
    [y1, x1] = c1.split(" ")
    [y2, x2] = c2.split(" ")
    n = int(n)
    m = int(m)
    gg = n*m
    y1 = int(y1)
    y2 = int(y2)
    x1 = int(x1)
    x2 = int(x2)
    yy = abs(y1-y2)
    xx = abs(x1-x2)
    tg = yy + xx
    money = int(p * tg)
    if money < 1000:
        print(money)
        print("You saved the group")
    else:
        print(money)
        print("Let go of the group")
