# syntax=docker/dockerfile:1

FROM golang:1.22
WORKDIR /app

# Deps
COPY go.mod go.sum ./
RUN go mod download

# App
COPY *.go ./
RUN CGO_ENABLED=0 GOOS=linux go build -o /app/pass-api

# Migrations
COPY migrations /app/migrations/

CMD ["/app/pass-api"]