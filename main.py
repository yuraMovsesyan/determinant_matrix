from random import choice
from copy import deepcopy
from threading import Thread


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

print("Determinant:", det(a))
print("Determinant:", det(a))
print("Determinant:", det(a))

t1 = DeterminantMultiflow(a)
t1.start()
t1.join()

t2 = DeterminantMultiflow(a)
t2.start()
t2.join()

t3 = DeterminantMultiflow(a)
t3.start()
t3.join()


print("t1:", t1.result, "t2:", "t2:", t2.result, "t3:", t3.result)
