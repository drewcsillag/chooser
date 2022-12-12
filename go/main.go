package main

import (
	"fmt"
	"strings"
)

type Chooser struct {
	executions *[][]int
	prechosen []int
	index int
	newchoices []int
}

type ChooserFunc func(*Chooser);

func stringifyExecution(ex []int) string {
	exs := []string{}
	for _, v := range ex {
		r := fmt.Sprintf("%d", v)
		exs = append(exs, r)
	}
	return fmt.Sprintf("[%s]", strings.Join(exs, ", "))
}
func stringifyExecutions(e [][]int) string {
	ex := []string{}
	for _, v := range e {
		ex = append(ex, stringifyExecution(v))
	}
	return fmt.Sprintf("[%s]", strings.Join(ex, ", "))
}

func (c *Chooser) ChooseIndex(numargs int) int {
	if c.index < len(c.prechosen) {
		retind := c.prechosen[c.index]
		c.index += 1
		return retind
	}

	var tslice []int
	for i := 1; i < numargs; i++ {
		newexec := append(append(c.prechosen, c.newchoices...), i)

		// ---
		// with the following two lines, you win
		// tslice = make([]int, len(newexec))
		// copy(tslice, newexec)

		// ----
		// with the following line uncommented, you lose
		tslice = newexec

		// reusing newexec w/o the copy results in unexpected data sharing
		// where it appears that slice returned from the above append is
		// the *same* slice, just with an altered value. >:( !!!!
		// So you wind up with executions like this:
		/// executions: ...[a,b,c]
		/// append [a,b,c,0]
		/// executions: ...[a,b,c], [a,b,c,0]
		/// append [a,b,c,1]
		/// executions: ...[a,b,c], [a,b,c,1], [a,b,c,1]
		///                 WTAF ----------^  that should be 0
		// This doesn't happen on shorter slices, but only when the new executions
		// grows to about 4-5 elements long
		// fmt.Printf("New execution: %s\n", stringifyExecution(tslice))
		*(*c).executions = append(*c.executions, tslice)
		// fmt.Printf("exes now: %s\n", stringifyExecutions(*c.executions))
	}
	c.newchoices = append(c.newchoices, 0)
	return 0
}

func (c *Chooser) Stop() {
	c.executions = nil
}

func RunChoices(f ChooserFunc) {
	executions := [][]int{nil}
	for len(executions) > 0 {
		// fmt.Printf("executions is: %s\n", stringifyExecutions(executions))
		prechosen := executions[len(executions)-1]
		executions = executions[:len(executions)-1]
		f(&Chooser{
			executions: &executions,
			prechosen: prechosen,
		})
	}
}

func intpick(c *Chooser, items *[]int) int {
	choice := c.ChooseIndex(len(*items))
	ret := (*items)[choice]
	// fmt.Printf("pick from %+v\n", *items)
	// fmt.Printf("chose index %d value %d\n", choice, ret)
	(*items) = append((*items)[:choice], (*items)[choice+1:]...)
	return ret
}

func test_magic(c *Chooser, counterbox *[]int) {
	left := []int{1,2,3,4,5,6,7,8,9}
	square := []int{0,0,0,0,0,0,0,0,0}
	(*counterbox)[1] += 1

	// fmt.Println("starting choose")

	square[0] = intpick(c, &left) 
	square[1] = intpick(c, &left) 
	square[2] = intpick(c, &left) 
	if square[0] + square[1] + square[2] != 15 {
		return
	}

	square[3] = intpick(c, &left) 
	square[4] = intpick(c, &left) 
	square[5] = intpick(c, &left) 

	if square[3] + square[4] + square[5] != 15 {
		return
	}

	square[6] = intpick(c, &left) 
	if square[0] + square[3] + square[6] != 15 ||
        square[2] + square[4] + square[6] != 15 {
		return
	}

	square[7] = intpick(c, &left) 
	if square[1] + square[4] + square[7] != 15 {
	    return
	}

	square[8] = intpick(c, &left) 
	if square[6] + square[7] + square[8] != 15 ||
		square[2] + square[5] + square[8] != 15 ||
		square[0] + square[4] + square[8] != 15 {
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
	
	for i := 0 ; i < 5; i++ {
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
	RunChoices(func (c *Chooser) { test_magic(c, &counterbox)})
	fmt.Printf("solutions and execs: %+v\n", counterbox)
	RunChoices(test_bin)
}