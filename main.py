from random import choice
from copy import deepcopy
from threading import Thread
import time

class DeterminantMultiflow(Thread):

    def __init__(self, arr):
        super().__init__()
        self.result = None

        size_arr = len(arr)
        if size_arr == 1: return arr[0][0]
        result = 0
        for index_x in range(size_arr):
            arr_deepcopy = deepcopy(arr)
            del (arr_deepcopy[0])
            for i in range(len(arr_deepcopy)):
                del (arr_deepcopy[i][index_x])
            result += arr[0][index_x] * (-1 if index_x & 1 else 1) * det(arr_deepcopy)

        self.result = result



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
    for x in y:
        print(str(x) + " " * (len(str(max_choice)) - len(str(x))), end='')
    print("")


start_time = time.time()

size_arr = len(a)
result = 0
DetMul = [DeterminantMultiflow(a)] * size_arr
for index_x in range(size_arr):
    arr_deepcopy = deepcopy(a)
    del (arr_deepcopy[0])
    for i in range(len(arr_deepcopy)):
        del (arr_deepcopy[i][index_x])
    DetMul[index_x] = DeterminantMultiflow(arr_deepcopy)
    DetMul[index_x].start()
    DetMul[index_x].join()

for index_x in range(size_arr):
    result += a[0][index_x] * (-1 if index_x & 1 else 1 ) * DetMul[index_x].result

print("Det:", result)

print("--- %s seconds ---" % (time.time() - start_time))