n = input()
s = input()
n = int(n)

bef_s_len = len(s) + 1
while bef_s_len != len(s):
    bef_s_len = len(s)
    s = s.replace('fox', '')

print(len(s))
