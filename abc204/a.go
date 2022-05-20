package main

import "fmt"

func main() {
	var x, y int
	fmt.Scan(&x, &y)

	if x == y {
		fmt.Printf("%d", x)
	} else {
		fmt.Printf("%d", 3-x-y)
	}
}
