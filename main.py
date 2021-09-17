from random import choice
from copy import deepcopy

def det(arr):
    size_arr = len(arr)
    if size_arr == 2: return arr[0][0] * arr[1][1] - arr[0][1] * arr[1][0]
    result = 0
    for index_x in range(size_arr):
        arr_deepcopy = deepcopy(arr)
        del (arr_deepcopy[0])
        for i in range(len(arr_deepcopy)):
            del (arr_deepcopy[i][index_x])
        result += arr[0][index_x] * (-1)**index_x * det(arr_deepcopy)
    return result

size = 5

a = [[choice(range(1, 10)) for i in range(size)] for j in range(size)]

print(a)
print(det(a))
