from random import choice
from copy import deepcopy

print("Input matrix size:")
size=int(input())
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

a = [[choice(range(1, 100)) for i in range(size)] for j in range(size)]

print("\n".join([" ".join([str(item) for item in row]) for row in a]))
print(det(a))
