
test = int(input())


def chess(moves):
    w = 8
    board = ["X"]*w**2
    k = 0
    for i in range(w):
        for j in range(w):
            if k >= moves:
                break
            board[w*i+j] = '.'
            k += 1
        if k >= moves:
            break
    board[0] = "O"
    for i in range(w):
        for j in range(w):
            print(board[w*i+j], end="")
        print()


for w in range(test):
    move = int(input())
    chess(move)









