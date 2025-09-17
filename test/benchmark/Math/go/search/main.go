package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func linearSearch(arr []int, target int) int {
	for i, val := range arr {
		if val == target {
			return i
		}
	}
	return -1
}

func main() {
	start := time.Now()
	startNs := start.UnixNano()

	fmt.Printf("Start time: %d ns\n", startNs)

	if len(os.Args) < 2 {
		fmt.Println("Usage: go run search.go <size>")
		return
	}

	size, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	arr := make([]int, size)
	for i := 0; i < size; i++ {
		arr[i] = i
	}

	target := size / 2
	result := linearSearch(arr, target)

	fmt.Printf("Found target %d at index %d in array of size %d\n", target, result, size)

	end := time.Now()
	endNs := end.UnixNano()
	durationNs := endNs - startNs
	durationS := float64(durationNs) / 1e9

	fmt.Printf("Execution time: %f seconds\n", durationS)
}
