package main

import "fmt"

func main() {
	var n int
	a := make([]int, n)

	fmt.Scan(&n)
	for i := 0; i < n; i++ {
		var tmp int
		fmt.Scan(&tmp)
		a = append(a, tmp)
	}

	res := 0
	outer: for {
		for i := 0; i < n; i++ {
			if a[i] % 2 != 0 {
				break outer
			}
			a[i] /= 2
		}
		res += 1
	}
	fmt.Println(res)
}
