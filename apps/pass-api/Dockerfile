# syntax=docker/dockerfile:1

FROM golang:1.22

ARG GIT_COMMIT_SHA
ARG GIT_REPOSITORY_URL
ENV DD_GIT_REPOSITORY_URL=${GIT_REPOSITORY_URL} 
ENV DD_GIT_COMMIT_SHA=${GIT_COMMIT_SHA}
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
