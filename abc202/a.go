package main

import "fmt"

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)

	res := 7*3 - (a + b + c)
	fmt.Printf("%d", res)
}
