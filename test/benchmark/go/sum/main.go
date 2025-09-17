package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func arraySum(arr []int) int64 {
	var sum int64
	for _, val := range arr {
		sum += int64(val)
	}
	return sum
}

func main() {
	start := time.Now()
	startNs := start.UnixNano()

	fmt.Printf("Start time: %d ns\n", startNs)

	if len(os.Args) < 2 {
		fmt.Println("Usage: go run sum.go <size>")
		return
	}

	size, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	arr := make([]int, size)
	for i := 0; i < size; i++ {
		arr[i] = i + 1
	}

	sum := arraySum(arr)

	fmt.Printf("Sum of array of size %d is: %d\n", size, sum)

	end := time.Now()
	endNs := end.UnixNano()
	durationNs := endNs - startNs
	durationS := float64(durationNs) / 1e9

	fmt.Printf("Execution time: %f seconds\n", durationS)
}
