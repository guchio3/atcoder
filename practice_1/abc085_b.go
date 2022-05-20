package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scan(&n)
	d := make(map[int]string)
	for i := 0; i < n; i++ {
		var d_i int
		fmt.Scan(&d_i)
		d[d_i] = ""
	}

	fmt.Println(len(d))
}
