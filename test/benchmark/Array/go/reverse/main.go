package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func reverseArray(arr []int) {
	start := 0
	end := len(arr) - 1
	for start < end {
		arr[start], arr[end] = arr[end], arr[start]
		start++
		end--
	}
}

func main() {
	start := time.Now()
	startNs := start.UnixNano()

	fmt.Printf("Start time: %d ns\n", startNs)

	if len(os.Args) < 2 {
		fmt.Println("Usage: go run reverse.go <size>")
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

	reverseArray(arr)

	fmt.Printf("Reversed array of size %d\n", size)

	end := time.Now()
	endNs := end.UnixNano()
	durationNs := endNs - startNs
	durationS := float64(durationNs) / 1e9

	fmt.Printf("Execution time: %f seconds\n", durationS)
}
