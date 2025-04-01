// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

package main

import (
	"fmt"
	"io"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
	httptrace "gopkg.in/DataDog/dd-trace-go.v1/contrib/net/http"
)

func respondToGetPasses(c *gin.Context) {
	ctx := c.Request.Context()
	logrus.WithContext(ctx).Info("Get all passes")

	var passes []mountainPass
	err := db.SelectContext(ctx, &passes, "SELECT * FROM mountain_pass")
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.IndentedJSON(http.StatusOK, passes)
}

func respondToGetSinglePass(c *gin.Context) {

	ctx := c.Request.Context()
	logrus.WithContext(ctx).Info("Fetching single pass")

	id := c.Param("id")
	var pass mountainPass
	err := db.GetContext(ctx, &pass, "SELECT * FROM mountain_pass WHERE id=$1", id)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Pass not found"})
		return
	}

	c.IndentedJSON(http.StatusOK, pass)
}

func respondToPostPasses(c *gin.Context) {
	ctx := c.Request.Context()
	logrus.WithContext(ctx).Info("Creating pass")

	var newPass mountainPass

	if err := c.BindJSON(&newPass); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid input"})
		return
	}

	// Use RETURNING to get the auto-generated ID
	var id int
	err := db.QueryRowContext(
		ctx,
		"INSERT INTO mountain_pass (name, country, ascent) VALUES ($1, $2, $3) RETURNING id",
		newPass.Name, newPass.Country, newPass.Ascent,
	).Scan(&id)

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	// Set the ID on the newPass struct
	newPass.ID = id

	c.IndentedJSON(http.StatusCreated, newPass)
}

func respondToDeletePass(c *gin.Context) {
	ctx := c.Request.Context()
	logrus.WithContext(ctx).Info("Deleting pass")

	id := c.Param("id")
	result, err := db.ExecContext(ctx, "DELETE FROM mountain_pass WHERE id=$1", id)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	rowsAffected, err := result.RowsAffected()
	if err != nil || rowsAffected == 0 {
		c.JSON(http.StatusNotFound, gin.H{"error": "Pass not found"})
		return
	}

	c.Status(http.StatusNoContent)
}

// Searches for passes by elevation range. This features an _extremely_ inefficient query
// that has been "designed" to stand out in APM.
func respondToSearchByElevation(c *gin.Context) {
	ctx := c.Request.Context()
	logrus.WithContext(ctx).Info("Searching passes by elevation range")

	min := c.Query("min")
	max := c.Query("max")

	if min == "" || max == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Both min and max elevation parameters are required"})
		return
	}

	query := `
		WITH pass_data AS (
			SELECT * FROM mountain_pass WHERE ascent BETWEEN $1 AND $2
		),
		all_countries AS (
			SELECT DISTINCT country FROM mountain_pass
		),
		all_combos AS (
			-- Intentional cartesian product to create huge result set
			SELECT p.*, c.country as related_country
			FROM pass_data p
			CROSS JOIN all_countries c
		)
		SELECT 
			p.*,
			(SELECT COUNT(*) FROM mountain_pass mp 
			 WHERE mp.country = p.country) as country_total,
			(SELECT string_agg(mp.name, ', ') 
			 FROM mountain_pass mp 
			 WHERE mp.country = p.country 
			   AND mp.name LIKE CONCAT('%', SUBSTRING(p.name FROM 1 FOR 1), '%')
			) as similar_passes,
			(SELECT COUNT(*) FROM mountain_pass mp 
			 WHERE mp.name ~ ('^' || SUBSTRING(p.name FROM 1 FOR 3) || '.*')
			) as regex_matches,
			(SELECT string_agg(mp.name, ', ') 
			 FROM mountain_pass mp 
			 WHERE mp.country = p.related_country
			) as same_country_passes
		FROM all_combos p
		ORDER BY LENGTH(p.name) * p.ascent * COALESCE(p.longitude, 0) * COALESCE(p.latitude, 0) DESC
	`

	// Create a struct that extends mountainPass to include the additional fields
	var passes []struct {
		mountainPass
		RelatedCountry    string `db:"related_country" json:"related_country"`
		CountryTotal      int    `db:"country_total" json:"country_total"`
		SimilarPasses     string `db:"similar_passes" json:"similar_passes"`
		RegexMatches      int    `db:"regex_matches" json:"regex_matches"`
		SameCountryPasses string `db:"same_country_passes" json:"same_country_passes"`
	}

	// Add intentional delay to make it even slower
	// Use a transaction to hold a lock longer
	tx, err := db.BeginTxx(ctx, nil)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	defer tx.Rollback()

	// Execute a simple count query first to acquire shared locks
	var count int
	err = tx.GetContext(ctx, &count, "SELECT COUNT(*) FROM mountain_pass")
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	// Now run our expensive query
	err = tx.SelectContext(ctx, &passes, query, min, max)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	// Log information about the range size to help debug
	logrus.WithContext(ctx).Infof("Retrieved %d passes for range %s-%s", len(passes), min, max)

	c.IndentedJSON(http.StatusOK, passes)
}

// Fetches an image for a given pass from the pass-image-api service
// This endpoint demonstrates potential failures when dealing with external services
func respondToGetPassImage(c *gin.Context) {
	ctx := c.Request.Context()
	logrus.WithContext(ctx).Info("Fetching pass image")

	passID := c.Param("id")

	// First, verify that the pass exists
	var pass mountainPass
	err := db.GetContext(ctx, &pass, "SELECT * FROM mountain_pass WHERE id=$1", passID)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Pass not found"})
		return
	}

	// Check if we have coordinates
	if pass.Latitude == 0 || pass.Longitude == 0 {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Pass has no coordinates"})
		return
	}

	// Construct the URL to the image service
	// The actual image API takes lat/long/size rather than pass ID
	// Format: /images/{long}/{lat}/{size_px}
	imageSize := 500 // Default size in pixels
	imageServiceURL := "http://pass-image-api:8080/images/" +
		toString(pass.Longitude) + "/" +
		toString(pass.Latitude) + "/" +
		toString(imageSize)

	// Optional radius parameter
	radius := 1.5 // Slightly larger view than default
	imageServiceURL += "?radius=" + toString(radius)

	// Create a traced HTTP client with Datadog
	httpClient := httptrace.WrapClient(&http.Client{
		// No timeout - intentionally problematic
	})

	// Use instrumented HTTP client for request
	req, err := http.NewRequestWithContext(ctx, "GET", imageServiceURL, nil)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":   "Failed to create request",
			"details": err.Error(),
		})
		return
	}

	// Execute the HTTP request with tracing
	resp, err := httpClient.Do(req)
	if err != nil {
		// Will fail if the service is down or unreachable
		c.JSON(http.StatusServiceUnavailable, gin.H{
			"error":   "Failed to connect to image service",
			"details": err.Error(),
		})
		return
	}
	defer resp.Body.Close()

	// Check if the image service returned an error
	if resp.StatusCode != http.StatusOK {
		c.JSON(resp.StatusCode, gin.H{
			"error":  "Image service returned an error",
			"status": resp.Status,
		})
		return
	}

	// Read the image data - no maximum size limit, potential memory issues
	imageData, err := io.ReadAll(resp.Body)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":   "Failed to read image data",
			"details": err.Error(),
		})
		return
	}

	// Set the content type from the response
	contentType := resp.Header.Get("Content-Type")
	if contentType == "" {
		// Default to png as used in the Rust service
		contentType = "image/png"
	}

	// Return the image data
	c.Data(http.StatusOK, contentType, imageData)
}

// Helper function to convert various types to string
func toString(v interface{}) string {
	switch val := v.(type) {
	case int:
		return strconv.Itoa(val)
	case float64:
		return strconv.FormatFloat(val, 'f', 6, 64)
	default:
		return fmt.Sprintf("%v", v)
	}
}
