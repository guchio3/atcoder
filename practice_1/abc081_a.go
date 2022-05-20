package main

import (
	"fmt"
)

func main() {
	var s string
	fmt.Scan(&s)

	res := 0
	for _, s_i := range s {
		res += int(s_i) - int('0')
	}
	fmt.Println(res)
}
