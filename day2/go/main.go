package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	instructions, err := ioutil.ReadFile("input.test")
	if err != nil {
		fmt.Println("Error reading file", err)
		return
	}
	part1(string(instructions))
	part2(string(instructions))
}

func part1(instructions string) {
	curx, curd := 0, 0
	for _, instruction := range strings.Split(string(instructions), "\r\n") {
		direction, value := parseInstruction(instruction)
		switch direction {
		case "forward":
			curx += value
		case "down":
			curd += value
		case "up":
			curd -= value
		}
	}
	fmt.Printf("Day 2 - Part 1: %v\n", curx*curd)
}

func part2(instructions string) {
	curx, curd, cura := 0, 0, 0
	for _, instruction := range strings.Split(string(instructions), "\r\n") {
		direction, value := parseInstruction(instruction)
		switch direction {
		case "forward":
			curx += value
			curd += cura * value
		case "down":
			cura += value
		case "up":
			cura -= value
		}
	}
	fmt.Printf("Day 2 - Part 2: %v\n", curx*curd)
}

func parseInstruction(instruction string) (string, int) {
	is := strings.Split(instruction, " ")
	amount, _ := strconv.Atoi(is[1])
	return is[0], amount
}
