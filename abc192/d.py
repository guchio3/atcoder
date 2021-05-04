x = input()
m = int(input())

if len(x) == 1:
    print(m - int(x))
    exit(0)

d = 0
flg = False
max_digit = -1

for i in range(len(x)):
    if x[i] != '0':
        flg = True
    if flg:
        max_digit += 1
    d = max(d, int(x[i]))

d += 1
max_digit_num = int(x[len(x) - max_digit - 1])

res = 0
for i in range(d, 10000000):
    tmp = 1
    for j in range(max_digit):
        tmp *= i
    if max_digit_num * tmp > m:
        break
    else:
        res += 1
        d += 1

d -= 1

while True:
    tmp = 0
    dd = 1
    for i in range(max_digit + 1):
        tmp += int(x[i]) * dd
        if tmp > m:
            break
        dd *= d
    if tmp > m:
        res -= 1
    else:
        break
    d -= 1

# tmp = 0
# dd = 1
# for i in range(max_digit + 1):
#     tmp += int(x[i]) * dd
#     if tmp > m:
#         break
#     dd *= d
# if tmp > m:
#     res -= 1


print(res)
