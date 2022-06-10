package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

var reader = NewReader()

func shift(slice []int) []int {
	c_slice := make([]int, len(slice))
	copy(c_slice, slice)
	sort.Ints(c_slice)
	cc_slice := make([]int, 0)
	cc_slice = append(cc_slice, c_slice[0])
	for i := range c_slice {
		if i == 0 || c_slice[i] == c_slice[i-1] {
			continue
		}
		cc_slice = append(cc_slice, c_slice[i])
	}

	v_map := make(map[int]int, 0)
	for i, x_i := range cc_slice {
		v_map[x_i] = i + 1
	}
	for i, slice_i := range slice {
		slice[i] = v_map[slice_i]
	}

	return slice
}

func main() {
	_, _, n := reader.IntTriple()

	a := make([]int, 0)
	b := make([]int, 0)
	for i := 0; i < n; i++ {
		a_i, b_i := reader.IntPair()
		a = append(a, a_i)
		b = append(b, b_i)
	}
	a = shift(a)
	b = shift(b)

	for i := 0; i < n; i++ {
		fmt.Printf("%d %d\n", a[i], b[i])
	}
}

// Read
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

func (r Reader) IntTriple() (int, int, int) {
	v := strings.Split(r.String(), " ")
	v0, _ := strconv.Atoi(v[0])
	v1, _ := strconv.Atoi(v[1])
	v2, _ := strconv.Atoi(v[2])
	return v0, v1, v2
}

func (r Reader) IntQuad() (int, int, int, int) {
	v := strings.Split(r.String(), " ")
	v0, _ := strconv.Atoi(v[0])
	v1, _ := strconv.Atoi(v[1])
	v2, _ := strconv.Atoi(v[2])
	v3, _ := strconv.Atoi(v[3])
	return v0, v1, v2, v3
}

func (r Reader) Ints(n int) []int {
	ans := make([]int, 0, n)
	for _, v := range strings.Split(r.String(), " ") {
		v, _ := strconv.Atoi(v)
		ans = append(ans, v)
	}
	return ans
}

func (r Reader) StringPair() (string, string) {
	v := strings.Split(r.String(), " ")
	return v[0], v[1]
}

func (r Reader) Strings(n int) []string {
	res := make([]string, 0, n)
	for _, v := range strings.Split(r.String(), " ") {
		res = append(res, v)
	}
	return res
}
