package main

import (
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"time"
)

func bubbleSort(arr []int) {
	n := len(arr)
	for i := 0; i < n-1; i++ {
		for j := 0; j < n-i-1; j++ {
			if arr[j] > arr[j+1] {
				arr[j], arr[j+1] = arr[j+1], arr[j]
			}
		}
	}
}

func main() {
	start := time.Now()
	startNs := start.UnixNano()

	fmt.Printf("Start time: %d ns\n", startNs)

	if len(os.Args) < 2 {
		fmt.Println("Usage: go run sort.go <size>")
		return
	}

	size, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	arr := make([]int, size)
	for i := 0; i < size; i++ {
		arr[i] = rand.Intn(1000)
	}

	bubbleSort(arr)

	fmt.Printf("Sorted array of size %d\n", size)

	end := time.Now()
	endNs := end.UnixNano()
	durationNs := endNs - startNs
	durationS := float64(durationNs) / 1e9

	fmt.Printf("Execution time: %f seconds\n", durationS)
}
