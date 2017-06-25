# Python Part

Source code of the Python part of my [Consuming and Publishing Celery Tasks in
C++ via
AMQP](https://blog.petrzemek.net/2017/06/25/consuming-and-publishing-celery-tasks-in-cpp-via-amqp/)
blog post.

## Requirements

* A running [RabbitMQ](https://www.rabbitmq.com/) server.
* Python 3.

## Installation

* `$ python -m venv virtualenv`
* `$ source virtualenv/bin/activate`
* `$ python -m pip pip install celery` (or just `$ pip install celery`
  if the path to the current working directory [is not too
  long](https://github.com/pypa/pip/issues/1773))

The project was successfully tested with Python 3.6.1, Celery 4.0.2, and
RabbitMQ 3.6.10 on 64b Arch Linux.

## Run

To send a request to call `hello()` in a worker with the given arguments, use

```text
python hello.py NAME AGE
```

To start a Celery worker (Python), use

```text
python -m celery -A tasks worker --loglevel=info
```

To stop the worker, press `Ctrl-C`.
