s = input()

if len(s) == 1:
    if int(s) % 8 == 0:
        print("Yes")
    else:
        print("No")
    exit()
if len(s) == 2:
    if int(s) % 8 == 0 or int(s[::-1]) % 8 == 0:
        print("Yes")
    else:
        print("No")
    exit()

candidates = []
for i in range(1000):
    if i % 8 == 0:
        if i < 10:
            str_num = f'00{i}'
        elif i < 100:
            str_num = f'0{i}'
        else:
            str_num = f'{i}'

        str_num_dict = {}
        for str_num_i in str_num:
            if str_num_i in str_num_dict:
                str_num_dict[str_num_i] += 1
            else:
                str_num_dict[str_num_i] = 1
        candidates.append(str_num_dict)


# candidates = set(candidates)

s_counter = {}
for s_i in str(s):
    if s_i in s_counter:
        s_counter[s_i] += 1
    else:
        s_counter[s_i] = 1

for candidate in candidates:
    for c_key in candidate:
        if c_key not in s_counter:
            break
        else:
            if s_counter[c_key] < candidate[c_key]:
                break
    else:
        print("Yes")
        exit()
print("No")
