package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"time"
)

func main() {

	now := time.Now()
	startNs := now.UnixNano() // epoch nanoseconds

	fmt.Printf("Start time: %d ns\n", startNs)

	if len(os.Args) < 2 {
		fmt.Println("Usage: go run cosine.go <angle_in_radians>")
		return
	}

	angle, err := strconv.ParseFloat(os.Args[1], 64)
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	result := math.Sin(angle)
	fmt.Printf("Sine of %f radians is: %f\n", angle, result)

	end := time.Now()
	endNs := end.UnixNano()
	durationNs := endNs - startNs
	durationS := float64(durationNs) / 1e9

	fmt.Printf("Execution time: %f seconds\n", durationS)
}
