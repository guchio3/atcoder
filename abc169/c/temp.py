in_vec = input().split()
a = int(in_vec[0])
b = float(in_vec[1])

# print(a * (b * 100) // 100)
# res = a * (b // 1) + a * (b - b // 1)
res = b * 100 * a
res /= 100
print(res)
