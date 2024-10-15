// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

package main

import (
	"github.com/gin-gonic/gin"
	"math/big"
	"math/rand"
	"net/http"
)

// Recursive helper function to check if a number is prime.This function is slow,
// and will blow through the stack, which is a great opportunity for us to diagnose runtime issues!.
// See what /primes/v1/67280421310721 does vs /primes/v2/67280421310721
func isPrimeRecursive(n *big.Int) bool {
	return isPrimeImpl(n, big.NewInt(2))
}

func isPrimeImpl(n *big.Int, divisor *big.Int) bool {
	one := big.NewInt(1)

	if n.Cmp(one) <= 0 {
		return false
	}

	divisorSquare := new(big.Int).Mul(divisor, divisor)
	if divisorSquare.Cmp(n) > 0 {
		return true
	}

	remainder := new(big.Int).Mod(n, divisor)
	if remainder.Cmp(big.NewInt(0)) == 0 {
		return false
	}

	return isPrimeImpl(n, new(big.Int).Add(divisor, one))
}

// Same as above but rewritten iteratively so that we don't blow through the stack. This should
// run but will be slow for large numbers.
func isPrimeIterative(n *big.Int) bool {
	one := big.NewInt(1)

	if n.Cmp(one) <= 0 {
		return false
	}

	divisor := big.NewInt(2)
	for {
		divisorSquare := new(big.Int).Mul(divisor, divisor)
		if divisorSquare.Cmp(n) > 0 {
			return true
		}

		remainder := new(big.Int).Mod(n, divisor)
		if remainder.Cmp(big.NewInt(0)) == 0 {
			return false
		}

		divisor.Add(divisor, one)
	}
}

// Endpoint to check if a number is prime
func makeRespondToCheckPrime(iterative bool) func(c *gin.Context) {
	// Choose the appropriate isPrimeRecursive function
	var isPrimeFunc func(*big.Int) bool
	if iterative {
		isPrimeFunc = isPrimeIterative
	} else {
		isPrimeFunc = isPrimeRecursive
	}

	// Return the function that handles the request
	return func(c *gin.Context) {
		numStr := c.Param("num")
		num := new(big.Int)
		_, ok := num.SetString(numStr, 10)
		if !ok || num.Sign() <= 0 {
			c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid number"})
			return
		}

		// Simulate a random failure
		if rand.Float32() < 0.3 { // 30% chance of failure
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Internal server error"})
			return
		}

		result := isPrimeFunc(num)
		c.JSON(http.StatusOK, gin.H{"number": num.String(), "is_prime": result})
	}
}
