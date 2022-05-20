package main

import "fmt"

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)

	if a == b {
		fmt.Printf("%d", c)
	} else if b == c {
		fmt.Printf("%d", a)
	} else if a == c {
		fmt.Printf("%d", b)
	} else {
		fmt.Printf("%d", 0)
	}
}
