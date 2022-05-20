package main

import "fmt"

func main() {
	var n, y int
	fmt.Scan(&n, &y)
	for i := 0; i <= n; i++ {
		for j := 0; i+j <= n; j++ {
			k := n - i - j
			sum := 1000*i + 5000*j + 10000*k
			if sum == y {
				fmt.Printf("%d %d %d", k, j, i)
				return
			}
		}
	}
	fmt.Printf("-1 -1 -1")
}
