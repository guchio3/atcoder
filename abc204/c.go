package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Read
var reader = NewReader()

type Reader struct {
	r *bufio.Reader
}

func NewReader() Reader {
	return Reader{bufio.NewReader(os.Stdin)}
}

func (r Reader) String() string {
	line, _ := reader.r.ReadString('\n')
	return strings.TrimRight(line, "\r\n")
}

func (r Reader) Int() int {
	line := r.String()
	v, _ := strconv.Atoi(line)
	return v
}

func (r Reader) IntPair() (int, int) {
	v := strings.Split(r.String(), " ")
	v0, _ := strconv.Atoi(v[0])
	v1, _ := strconv.Atoi(v[1])
	return v0, v1
}

func s2i(s string) int {
	v, ok := strconv.Atoi(s)
	if ok != nil {
		panic("Faild : " + s + " can't convert to int")
	}
	return v
}

func main() {
	n, m := reader.IntPair()

	paths := make([][]int, n)

	for i := 0; i < m; i++ {
		a_i, b_i := reader.IntPair()
		paths[a_i-1] = append(paths[a_i-1], b_i-1)
	}

	res := n
	for i := 0; i < n; i++ {
		queue := make([]int, 0)
		used := make([]int, n)
		used[i] = 1
		queue = append(queue, i)
		for len(queue) > 0 {
			// cur := queue[0]
			// queue = queue[1:] // pop
			cur := queue[len(queue)-1]
			queue = queue[:len(queue)-1] // pop
			for _, j := range paths[cur] {
				if used[j] == 0 {
					queue = append(queue, j)
					used[j] = 1
					res += 1
				}
			}
		}
	}

	fmt.Printf("%d", res)

}
