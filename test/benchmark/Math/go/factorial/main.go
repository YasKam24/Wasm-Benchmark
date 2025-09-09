package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func factorial(n int) float64 {
	result := 1.0
	for i := 2; i <= n; i++ {
		result *= float64(i)
	}
	return result
}

func main() {
	start := time.Now()
	startNs := start.UnixNano() // epoch nanoseconds

	fmt.Printf("Start time: %d ns\n", startNs)

	if len(os.Args) < 2 {
		fmt.Println("Usage: go run factorial_sum.go <n>")
		return
	}

	n, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	sum := 0.0
	for i := 1; i <= n; i++ {
		sum += factorial(i)
	}

	fmt.Printf("Sum of factorials from 1 to %d is: %.0f\n", n, sum)

	end := time.Now()
	endNs := end.UnixNano()
	durationNs := endNs - startNs
	durationS := float64(durationNs) / 1e9

	fmt.Printf("Execution time: %f seconds\n", durationS)
}
