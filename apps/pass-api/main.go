package main

import (
	"errors"
	"github.com/gin-gonic/gin"
	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database/postgres"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	"github.com/jmoiron/sqlx"
	"github.com/lib/pq"
	_ "github.com/lib/pq"
	"github.com/sirupsen/logrus"
	sqltrace "gopkg.in/DataDog/dd-trace-go.v1/contrib/database/sql"
	gintrace "gopkg.in/DataDog/dd-trace-go.v1/contrib/gin-gonic/gin"
	sqlxtrace "gopkg.in/DataDog/dd-trace-go.v1/contrib/jmoiron/sqlx"
	dd_logrus "gopkg.in/DataDog/dd-trace-go.v1/contrib/sirupsen/logrus"
	"gopkg.in/DataDog/dd-trace-go.v1/ddtrace/tracer"
	"gopkg.in/DataDog/dd-trace-go.v1/profiler"
	"log"
	"os"
)

type mountainPass struct {
	ID        int     `json:"id"`
	Name      string  `json:"name"`
	Country   string  `json:"country"`
	Ascent    int     `json:"ascent"`
	Latitude  float64 `json:"latitude"`
	Longitude float64 `json:"longitude"`
	Category  string  `json:"climb_category"`
}

var db *sqlx.DB

func initDB(user string, pass string, db_name string, hostname string) error {

	// Enable tracing for the DB
	sqltrace.Register("postgres", &pq.Driver{}, sqltrace.WithServiceName("pass-api"))

	// Connect
	var err error
	db, err = sqlxtrace.Open("postgres", "postgres://"+user+":"+pass+"@"+hostname+"/"+db_name+"?sslmode=disable")
	if err != nil {
		logrus.Fatalf("Error connecting to database: %v", err)
	}

	// Start Postgres migration driver
	driver, err := postgres.WithInstance(db.DB, &postgres.Config{})
	if err != nil {
		logrus.Fatalf("Failed starting DB Driver: %v", err)
	}
	m, err := migrate.NewWithDatabaseInstance(
		"file://migrations",
		"postgres", driver)

	if err != nil {
		logrus.Fatalf("Error initializing migrations: %v", err)
	}

	if err := m.Up(); err != nil && !errors.Is(err, migrate.ErrNoChange) {
		logrus.Fatalf("Migration failed: %v", err)
	}

	// All good!
	logrus.Println("Successfully initialized database")
	return nil
}

func main() {

	// Setup log formatting and hook up datadog span injection
	logrus.SetFormatter(&logrus.JSONFormatter{})
	logrus.AddHook(&dd_logrus.DDContextLogHook{})

	// Start the datadog tracer
	tracer.Start()
	defer tracer.Stop()

	// Setup the profiler
	err := profiler.Start(
		//profiler.WithService("pass-api"),
		//profiler.WithEnv("dev"),
		//profiler.WithVersion("localdev"),

		profiler.WithProfileTypes(
			profiler.CPUProfile,
			profiler.HeapProfile))
	if err != nil {
		logrus.Fatalf("Error starting profiler: %v", err)
	}
	defer profiler.Stop()

	// Read connection details from the environment
	host := os.Getenv("POSTGRES_HOST")
	user := os.Getenv("POSTGRES_USER")
	pass := os.Getenv("POSTGRES_PASSWORD")
	dbName := os.Getenv("POSTGRES_DB")
	if host == "" || user == "" || pass == "" || dbName == "" {
		log.Fatal("Missing environment variables - need POSTGRES_HOST, POSTGRES_USER, POSTGRES_PASSWORD, POSTGRES_DB")
	}

	// Start the DB
	err = initDB(user, pass, dbName, host)
	if err != nil {
		log.Fatalf("Error initializing database: %v", err)
	}

	// Start the server
	router := gin.Default()
	router.Use(gintrace.Middleware("pass-api"))

	router.GET("/passes", respondToGetPasses)
	router.GET("/passes/:id", respondToGetSinglePass)
	router.POST("/passes", respondToPostPasses)
	router.DELETE("/passes/:id", respondToDeletePass)
	router.GET("/primes/v1/:num", makeRespondToCheckPrime(false))
	router.GET("/primes/v2/:num", makeRespondToCheckPrime(true))
	router.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"ok": true,
		})
	})
	
	err = router.Run(":8080")
	if err != nil {
		log.Fatalf("Error starting server: %v", err)
	}
}
