s = input()

res = 0
dp_dict = {}

for i_upper in range(0, len(s)):
    temp_mod = 0
    temp_dp_dict = {}
    for i_lower in range(i_upper, len(s)):
        temp_mod = (temp_mod * 10 + int(s[i_lower])) % 2019

        if temp_mod == 0:
            if i_lower in dp_dict:
                res += dp_dict[i_lower]
                for key in temp_dp_dict:
                    temp_dp_dict[key] += dp_dict[i_lower]
                break
            res += 1
            temp_dp_dict[i_lower] = 0
            for key in temp_dp_dict:
                temp_dp_dict[key] += 1

    for key in temp_dp_dict:
        dp_dict[key] = temp_dp_dict[key]

print(res)
