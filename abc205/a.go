package main

import "fmt"

func main() {
	var a, b float32
	fmt.Scan(&a, &b)

	fmt.Printf("%g", a*b/100)
}
