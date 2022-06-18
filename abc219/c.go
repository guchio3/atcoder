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

func main() {
	x := reader.String()
	local_dict := make(map[rune]rune, 0)
	rev_local_dict := make(map[rune]rune, 0)
	for i, x_i := range x {
		local_dict[x_i] = rune(i)
		rev_local_dict[rune(i)] = x_i
	}
	n := reader.Int()
	s := make([]string, 0)
	for i := 0; i < n; i++ {
		s_i := reader.String()
		local_rune_s_i := make([]rune, 0)
		for _, s_ij := range s_i {
			local_rune_s_i = append(local_rune_s_i, local_dict[s_ij])
		}
		s = append(s, string(local_rune_s_i))
	}
	sort.Strings(s)

	for _, s_i := range s {
		tmp_res := make([]rune, 0)
		for _, s_ij := range s_i {
			tmp_res = append(tmp_res, rev_local_dict[s_ij])
		}
		fmt.Printf("%s\n", string(tmp_res))
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
