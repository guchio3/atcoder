package main

import "fmt"

func checkDigitSum10(num int) int {
	res := 0
	for num > 0 {
		res += num % 10
		num /= 10
	}
	return res
}

func main() {
	var n, a, b int
	fmt.Scan(&n, &a, &b)

	res := 0
	for i := 0; i <= n; i++ {
		digit_sum := checkDigitSum10(i)
		if a <= digit_sum && digit_sum <= b {
			res += i
		}
	}

	fmt.Println(res)
}
