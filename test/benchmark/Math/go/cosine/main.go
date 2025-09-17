package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"time"
)

func main() {
	// record start time
	start := time.Now()
	startNs := start.UnixNano() // epoch nanoseconds

	fmt.Printf("Start time: %d ns\n", startNs)

	// check arguments
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run cosine.go <angle_in_radians>")
		return
	}

	// parse argument
	angle, err := strconv.ParseFloat(os.Args[1], 64)
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	// do the computation
	result := math.Cos(angle)
	fmt.Printf("Cosine of %f radians is: %f\n", angle, result)

	// record end time
	end := time.Now()
	endNs := end.UnixNano()

	// compute duration
	durationNs := endNs - startNs
	durationS := float64(durationNs) / 1e9

	fmt.Printf("Execution time: %f seconds\n", durationS)
}
