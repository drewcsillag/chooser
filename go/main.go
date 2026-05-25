package main

import (
	"fmt"
)

type Chooser struct {
	executions  *[][]int
	prechosen   []int
	index       int
	new_choices []int
}

type ChooserFunc func(*Chooser)

func (c *Chooser) ChooseIndex(numargs int) int {
	// Handle pre-chosen case
	if c.index < len(c.prechosen) {
		retind := c.prechosen[c.index]
		c.index++
		return retind
	}

	// Generate new choices for all possible indices except 0
	for i := 1; i < numargs; i++ {
		// Create new choice path by combining existing choices with new one
		newPath := make([]int, len(c.prechosen))
		copy(newPath, c.prechosen)
		newPath = append(append(newPath, c.new_choices...), i)
		*c.executions = append(*c.executions, newPath)
	}

	// Add 0 to current choice path and return it
	c.new_choices = append(c.new_choices, 0)
	return 0
}

func (c *Chooser) Stop() {
	c.executions = nil
}

func executeChoicePath(f ChooserFunc, choicePath []int, remainingPaths *[][]int) {
	chooser := &Chooser{
		executions:  remainingPaths,
		prechosen:   choicePath,
		new_choices: make([]int, 0),
	}
	f(chooser)
}

func RunChoices(f ChooserFunc) {
	choicePaths := [][]int{nil}

	for len(choicePaths) > 0 {
		lastIndex := len(choicePaths) - 1
		currentPath := choicePaths[lastIndex]
		choicePaths = choicePaths[:lastIndex]

		executeChoicePath(f, currentPath, &choicePaths)
	}
}

func intpick(c *Chooser, items *[]int) int {
	choice := c.ChooseIndex(len(*items))
	ret := (*items)[choice]
	(*items) = append((*items)[:choice], (*items)[choice+1:]...)
	return ret
}

func test_magic(c *Chooser, counterbox *[]int) {
	left := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
	square := []int{0, 0, 0, 0, 0, 0, 0, 0, 0}
	(*counterbox)[1] += 1

	// fmt.Println("starting choose")

	square[0] = intpick(c, &left)
	square[1] = intpick(c, &left)
	square[2] = intpick(c, &left)
	if square[0]+square[1]+square[2] != 15 {
		return
	}

	square[3] = intpick(c, &left)
	square[4] = intpick(c, &left)
	square[5] = intpick(c, &left)

	if square[3]+square[4]+square[5] != 15 {
		return
	}

	square[6] = intpick(c, &left)
	if square[0]+square[3]+square[6] != 15 ||
		square[2]+square[4]+square[6] != 15 {
		return
	}

	square[7] = intpick(c, &left)
	if square[1]+square[4]+square[7] != 15 {
		return
	}

	square[8] = intpick(c, &left)
	if square[6]+square[7]+square[8] != 15 ||
		square[2]+square[5]+square[8] != 15 ||
		square[0]+square[4]+square[8] != 15 {
		return
	}
	fmt.Printf("[%d, %d, %d]\n", square[0], square[1], square[2])
	fmt.Printf("[%d, %d, %d]\n", square[3], square[4], square[5])
	fmt.Printf("[%d, %d, %d]\n", square[6], square[7], square[8])
	fmt.Println("")
	(*counterbox)[0] += 1

}

var emitted []bool

func test_bin(c *Chooser) {
	t := 0
	s := make([]int, 5)

	for i := 0; i < 5; i++ {
		s[i] = c.ChooseIndex(2)
		t *= 2
		t += s[i]
	}

	r := ""
	if emitted[t] {
		r = "FAIL"
	}
	emitted[t] = true

	fmt.Printf("[%d, %d, %d, %d, %d] = %d %s\n",
		s[0], s[1], s[2], s[3], s[4], t, r)
}

func main() {
	emitted = make([]bool, 32)
	for i := 0; i < 32; i++ {
		emitted[i] = false
	}
	counterbox := []int{0, 0}
	RunChoices(func(c *Chooser) { test_magic(c, &counterbox) })
	fmt.Printf("solutions and execs: %+v\n", counterbox)
	RunChoices(test_bin)
}
