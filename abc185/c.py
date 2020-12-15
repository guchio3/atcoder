from math import comb

ll = int(input())

print(comb(ll - 1, 11))


# res = 1
# for i in range(1, 12):
#     res *= (ll - i)
# tmp = 1
# for i in range(1, 12):
#     tmp *= i
# 
# print(res // tmp)
