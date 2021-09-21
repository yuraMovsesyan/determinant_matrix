from random import choice
import time
import itertools

size = int(input("Input matrix size: "))

min_choice = 1
max_choice = 10

def getInversion(arr):
    size_arr = len(arr)
    result_count = 0

    for i in range(size_arr):
        for j in range(i + 1, size_arr):
            if (arr[i] > arr[j]):
                result_count += 1

    return result_count

def det(arr):
    size_arr = len(arr)
    if size_arr == 1: return arr[0][0]
    result = 0

    for item in itertools.permutations(range(0, size_arr)):
        res = 1
        for id in range(size_arr):
            res *= arr[id][item[id]]
            if res == 0: break

        if res == 0: continue
        if getInversion(item) % 2 == 0:
            result += res
        else:
            result -= res
    return result

a = [[choice(range(min_choice, max_choice)) for i in range(size)] for j in range(size)]

for y in a:
    for x in y:
        print(str(x) + " " * (len(str(max_choice)) - len(str(x))), end='')
        if max_choice < 10: print("", end=' ')
    print("")

start_time = time.time()
print("Determinant:", det(a))
print("--- %s seconds ---" % (time.time() - start_time))
