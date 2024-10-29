# syntax=docker/dockerfile:1

FROM --platform=${BUILDPLATFORM} golang:1.22 AS builder

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
ARG TARGETPLATFORM
RUN CGO_ENABLED=0 GOOS=linux GOARCH="${TARGETPLATFORM#*/}" go build \
    -ldflags="-s -w" -trimpath -o /app/pass-api

# Migrations
COPY migrations /app/migrations/

# Release stage (minimal image)
FROM scratch

# Set the workdir
WORKDIR /app

COPY --from=builder /app/pass-api /app/pass-api
COPY --from=builder /app/migrations /app/migrations

CMD ["./pass-api"]
