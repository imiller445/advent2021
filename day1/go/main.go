package main

import (
	"bufio"
	"fmt"
	"io"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	depthsB, err := ioutil.ReadFile("input")
	if err != nil {
		fmt.Println("Error reading file", err)
		return
	}
	depths := ReadInts(strings.NewReader(string(depthsB)))
	part1(depths)
	part2(depths)
}

func part1(depths []int) {
	prev := int(^uint(0) >> 1)
	total := 0
	for _, i := range depths {
		if i > prev {
			total++
		}
		prev = i
	}
	fmt.Printf("Day 1 - Part 1: %v\n", total)
}

func part2(depths []int) {
	prev := sumOverSlice(depths[0:3])
	total := 0
	for i := 1; i < len(depths)-2; i++ {
		cur := sumOverSlice(depths[i : i+3])
		if cur > prev {
			total++
		}
		prev = cur
	}
	fmt.Printf("Day 1 - Part 2: %v\n", total)
}

func sumOverSlice(slice []int) int {
	sum := 0
	for _, i := range slice {
		sum += i
	}
	return sum
}

func ReadInts(r io.Reader) []int {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanWords)
	var result []int
	for scanner.Scan() {
		x, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return result
		}
		result = append(result, x)
	}

	return result
}
