from random import choice
import time

size = int(input("Input matrix size: "))

min_choice = 1
max_choice = 10

def getPermutations(arr, flag = False):
    if not flag:
        arr = list(arr)

    if len(arr) == 1:
        return [arr]

    iterate_perm = getPermutations(arr[1:], True)

    result_perm = []
    number = [arr[0]]

    for perm in iterate_perm:
        for i in range(0, len(perm) + 1):
            result_perm.append(perm[:i] + number + perm[i:])
    return result_perm


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

    for item in getPermutations(range(0, size_arr)):
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

print("\n".join([" ".join([(str(item) + " " * (len(str(max_choice)) - len(str(item)))) for item in row]) for row in a]))

start_time = time.time()
print("Determinant:", det(a))
print("--- %s seconds ---" % (time.time() - start_time))
