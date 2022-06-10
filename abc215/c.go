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

func contains(l []int, x int) bool {
	for _, l_i := range l {
		if l_i == x {
			return true
		}
	}
	return false
}

func main() {
	s, kk := reader.StringPair()
	k, _ := strconv.Atoi(kk)

	candidates := make([][]int, 0)
	queue := make([][]int, 0)
	for i := range s {
		queue_elem_i := []int{i}
		queue = append(queue, queue_elem_i)
	}
	for len(queue) > 0 {
		elem := queue[0]
		queue = queue[1:]

		if len(elem) == len(s) {
			candidates = append(candidates, elem)
			continue
		}

		for i := range s {
			if !contains(elem, i) {
				next_elem := make([]int, len(elem))
				copy(next_elem, elem)
				next_elem = append(next_elem, i)
				queue = append(queue, next_elem)
			}
		}
	}

	rune_s := []rune(s)

	str_candidates := make([]string, 0)
	for _, candidates_i := range candidates {
		rune_candidate := make([]rune, 0)
		for _, j := range candidates_i {
			rune_candidate = append(rune_candidate, rune_s[j])
		}
		str_candidates = append(str_candidates, string(rune_candidate))
	}

	sort.Strings(str_candidates)
	bef := ""
	for _, str_candidate := range str_candidates {
		if bef != str_candidate {
			k -= 1
			bef = str_candidate
		}
		if k == 0 {
			fmt.Printf("%s", str_candidate)
			return
		}
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
