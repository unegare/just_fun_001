FROM alpine:latest

RUN apk update && apk add --no-cache rust

WORKDIR /app

COPY ./20200418_task.rs .

RUN rustc ./20200418_task.rs

ENTRYPOINT ["/app/20200418_task"]
