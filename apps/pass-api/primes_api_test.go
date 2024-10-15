// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

package main

import (
	"math/big"
	"testing"
)

func TestIsPrime(t *testing.T) {
	tests := []struct {
		number   string
		expected bool
	}{
		{"2", true},   // 2 is prime
		{"3", true},   // 3 is prime
		{"4", false},  // 4 is not prime
		{"5", true},   // 5 is prime
		{"25", false}, // 25 is not prime (5 * 5)
		{"17", true},  // 17 is prime,
	}

	for _, test := range tests {
		num := new(big.Int)
		num.SetString(test.number, 10)

		result := isPrimeRecursive(num)
		if result != test.expected {
			t.Errorf("isPrimeRecursive(%s) = %v; expected %v", test.number, result, test.expected)
		}

		result = isPrimeIterative(num)
		if result != test.expected {
			t.Errorf("isPrimeIterative(%s) = %v; expected %v", test.number, result, test.expected)
		}
	}
}
