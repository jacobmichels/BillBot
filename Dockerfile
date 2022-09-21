FROM golang:1.19.1-alpine3.16 AS builder

WORKDIR /app

COPY go.mod .
COPY go.mod .

RUN go mod download

COPY . .

RUN go build -o /usr/bin/billbot ./cmd/BillBot

FROM alpine:3.16.2

COPY --from=builder /usr/bin/billbot /usr/bin/billbot

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

ENTRYPOINT [ "/usr/bin/billbot" ]