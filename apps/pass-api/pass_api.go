// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

package main

import (
	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
	"net/http"
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
