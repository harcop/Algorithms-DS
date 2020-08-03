import random

r = 8
row = r-1
c = 8


def new_cell(x, y):
    return {"value": ".", "isVisited": False, "x": x, "y": y}


def init_board():
    inner_board = {}
    for y in range(r):
        for x in range(c):
            inner_board[str(x)+str(y)] = new_cell(x, y)
    return inner_board


def print_board(board):
    for y in range(r):
        for x in range(c):
            print(board[str(x)+str(y)]["value"], end=" ")
        print()


def chess(move):
    icr = random.randint(0, row)
    icc = random.randint(0, row)
    icp = boarder[str(icr)+str(icc)]
    icp["value"] = 'O'
    vc = []
    current = icp
    i = 2
    while len(vc) < move:
        current["isVisited"] = True
        side_ne = neighbour(current)
        if str(current["x"])+str(current["y"]) not in vc:
            vc.append(str(current["x"])+str(current["y"]))
        if len(side_ne) is not 0:
            rand_neigh = random.randint(0, len(side_ne)-1)
            current = side_ne[rand_neigh]
            # print("current:", rand_neigh, current["x"], current["y"])
            i += 1
        else:
            rand_vc = random.randint(0, len(vc) - 1)
            current = boarder[vc[rand_vc]]

    for k in vc:
        i_neigh = neighbour(boarder[k])
        for j in i_neigh:
            boarder[str(j["x"])+str(j["y"])]["value"] = "X"

    print_board(boarder)


def neighbour(cell):
    x = int(cell["x"])
    y = int(cell["y"])
    neigh = []
    if y-1 >= 0:
        up_x = x
        up_y = y-1
        if boarder[str(up_x)+str(up_y)]["isVisited"] is False:
            neigh.append(boarder[str(up_x)+str(up_y)])
    if y-1 >= 0 and x+1 <= row:
        up_east_x = x+1
        up_east_y = y-1
        if boarder[str(up_east_x)+str(up_east_y)]["isVisited"] is False:
            neigh.append(boarder[str(up_east_x)+str(up_east_y)])
    if x+1 <= row:
        right_x = x+1
        right_y = y
        if boarder[str(right_x)+str(right_y)]["isVisited"] is False:
            neigh.append(boarder[str(right_x)+str(right_y)])
    if y+1 <= row and x+1 <= row:
        down_east_x = x+1
        down_east_y = y+1
        if boarder[str(down_east_x)+str(down_east_y)]["isVisited"] is False:
            neigh.append(boarder[str(down_east_x)+str(down_east_y)])
    if y+1 <= row:
        bottom_x = x
        bottom_y = y+1
        if boarder[str(bottom_x)+str(bottom_y)]["isVisited"] is False:
            neigh.append(boarder[str(bottom_x)+str(bottom_y)])
    if y+1 <= row and x-1 >= 0:
        down_west_x = x-1
        down_west_y = y+1
        if boarder[str(down_west_x)+str(down_west_y)]["isVisited"] is False:
            neigh.append(boarder[str(down_west_x)+str(down_west_y)])
    if x-1 >= 0:
        left_x = x-1
        left_y = y
        if boarder[str(left_x)+str(left_y)]["isVisited"] is False:
            neigh.append(boarder[str(left_x)+str(left_y)])
    if y-1 >= 0 and x-1 >= 0:
        up_west_x = x-1
        up_west_y = y-1
        if boarder[str(up_west_x)+str(up_west_y)]["isVisited"] is False:
            neigh.append(boarder[str(up_west_x)+str(up_west_y)])

    return neigh




# ###start below###
no_test = int(input("please enter number of test cases "))

for i in range(no_test):
    moves = int(input())
    boarder = init_board()
    chess(moves)


