package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var reader = NewReader()

type Point struct {
	x int
	y int
}

func rotate(x [][]rune) [][]rune {
	height := len(x)
	width := len(x[0])
	res := make([][]rune, 0)
	for i := 0; i < height; i++ {
		res = append(res, make([]rune, width))
	}

	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			res[j][height-1-i] = x[i][j]
		}
	}

	return res
}

func is_same_shape(a, b [][]rune) bool {
	a_start := Point{x: -1, y: -1}
outer_a:
	for i := 0; i < len(a); i++ {
		for j := 0; j < len(a[0]); j++ {
			if a[i][j] == '#' {
				a_start = Point{x: i, y: j}
				break outer_a
			}
		}
	}
	if a_start.x == -1 {
		fmt.Printf("AAAAAAAAAAAAAAAAAAAA")
	}

	b_start := Point{x: -1, y: -1}
outer_b:
	for i := 0; i < len(b); i++ {
		for j := 0; j < len(b[0]); j++ {
			if b[i][j] == '#' {
				b_start = Point{x: i, y: j}
				break outer_b
			}
		}
	}
	if b_start.x == -1 {
		fmt.Printf("AAAAAAAAAAAAAAAAAAAA")
	}

	diff := Point{x: a_start.x - b_start.x, y: a_start.y - b_start.y}

	for i := 0; i < len(a); i++ {
		for j := 0; j < len(a[0]); j++ {
			if a[i][j] == '#' {
				b_x := i - diff.x
				b_y := j - diff.y
				if !(0 <= b_x && b_x < len(a) && 0 <= b_y && b_y < len(a[0]) && b[b_x][b_y] == '#') {
					return false
				}
			}
			if b[i][j] == '#' {
				a_x := i + diff.x
				a_y := j + diff.y
				if !(0 <= a_x && a_x < len(a) && 0 <= a_y && a_y < len(a[0]) && a[a_x][a_y] == '#') {
					return false
				}
			}
		}
	}

	return true
}

func main() {
	n := reader.Int()

	s := make([][]rune, 0)
	for i := 0; i < n; i++ {
		s_i := reader.String()

		s_i_slice := make([]rune, 0)
		for _, s_i_j := range s_i {
			s_i_slice = append(s_i_slice, s_i_j)
		}
		s = append(s, s_i_slice)
	}
	t := make([][]rune, 0)
	for i := 0; i < n; i++ {
		t_i := reader.String()
		t_i_slice := make([]rune, 0)
		for _, t_i_j := range t_i {
			t_i_slice = append(t_i_slice, t_i_j)
		}
		t = append(t, t_i_slice)
	}

	if is_same_shape(s, t) {
		fmt.Printf("Yes")
		return
	}
	for i := 0; i < 3; i++ {
		t = rotate(t)
		if is_same_shape(s, t) {
			fmt.Printf("Yes")
			return
		}
	}
	fmt.Printf("No")
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
