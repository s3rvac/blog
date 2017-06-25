# C++ Part

Source code of the C++ part of my [Consuming and Publishing Celery Tasks in C++
via
AMQP](https://blog.petrzemek.net/2017/06/25/consuming-and-publishing-celery-tasks-in-cpp-via-amqp/)
blog post.

## Requirements

* A running [RabbitMQ](https://www.rabbitmq.com/) server.
* A C++11 compiler.
* [`librabbitmq-c`](https://github.com/alanxz/rabbitmq-c)
* [Boost](http://www.boost.org/)

## Build

* `mkdir build`
* `cd build`
* `cmake ..`
* `make`

The project was successfully tested with GCC 7.1, CMake 3.8.2, RabbitMQ 3.6.10,
`librabbitmq-c` 0.8.0, and Boost 1.64 on 64b Arch Linux.

## Run

It is assumed that you are in the `cpp` directory.

To send a request to call `hello()` in a worker with the given arguments, use

```text
build/hello NAME AGE
```

To start a worker (C++), use

```text
build/worker
```

To stop the worker, press `Ctrl-C`.
