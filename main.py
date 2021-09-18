from random import choice
from copy import deepcopy

size = int(input("Input matrix size: "))

min_choice = 1
max_choice = 100

def det(arr):
    size_arr = len(arr)
    if size_arr == 1: return arr[0][0]
    result = 0
    for index_x in range(size_arr):
        arr_deepcopy = deepcopy(arr)
        del (arr_deepcopy[0])
        for i in range(len(arr_deepcopy)):
            del (arr_deepcopy[i][index_x])
        result += arr[0][index_x] * (-1 if index_x & 1 else 1 ) * det(arr_deepcopy)
    return result

a = [[choice(range(min_choice, max_choice)) for i in range(size)] for j in range(size)]

for y in a:
    print("|", end="")
    for x in y:
        print(str(x) + " " * (len(str(max_choice)) - len(str(x))), end='')
    print("|")
print("Determinant:", det(a))
