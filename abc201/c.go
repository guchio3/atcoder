package main

import "fmt"

func conbination(n, k int) int {
	if n == 0 || k == 0 {
		return 1
	}
	res := 1
	for i := 0; i < k; i++ {
		res *= (n - i)
	}
	for i := 1; i <= k; i++ {
		res /= i
	}
	return res
}

func main() {
	var s string
	fmt.Scan(&s)

	maru := make(map[int]bool, 0)
	var ques [10]bool
	for i, s_i := range s {
		if s_i == 'o' {
			maru[i] = false
		} else if s_i == '?' {
			ques[i] = true
		}
	}

	nums := make([]int, 0)
	for i := 0; i < 10; i++ {
		nums = append(nums, i)
	}

	res := 0
	for _, num4 := range nums {
		for _, num3 := range nums {
			for _, num2 := range nums {
			outer:
				for _, num1 := range nums {
					local_maru := make(map[int]bool, 0)
					for val, num := range maru {
						local_maru[val] = num
					}
					for _, num := range []int{num4, num3, num2, num1} {
						is_ques := ques[num]
						if _, ok := local_maru[num]; ok {
							local_maru[num] = true
						} else if !is_ques {
							continue outer
						}
					}
					// check if all marus are used
					for _, val := range local_maru {
						if !val {
							continue outer
						}
					}
					res += 1
				}
			}
		}
	}
	fmt.Printf("%d", res)
}
