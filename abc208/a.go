package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)

	if 1 * a <= b && b <= 6 * a {
		fmt.Printf("Yes")
	} else {
		fmt.Printf("No")
	}
}