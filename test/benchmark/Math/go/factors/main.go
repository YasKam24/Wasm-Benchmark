package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func main() {
	now := time.Now()
	StartNs := now.UnixNano() // epoch nanoseconds

	fmt.Printf("Start time: %d ns\n", StartNs)

	if len(os.Args) < 2 {
		fmt.Println("Usage: wasmedge factors.wasm <number>")
		return
	}

	// Read input from command line
	n, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	// Compute and print factors
	fmt.Printf("Factors of %d: ", n)
	for i := 1; i <= n; i++ {
		if n%i == 0 {
			fmt.Printf("%d ", i)
		}
	}
	fmt.Println()

	end := time.Now()
	endNs := end.UnixNano()
	durationNs := endNs - StartNs
	durationS := float64(durationNs) / 1e9

	fmt.Printf("Execution time: %f seconds\n", durationS)
}
