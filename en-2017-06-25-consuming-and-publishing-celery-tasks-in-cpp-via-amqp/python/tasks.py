#
# Creates a main application and registers a hello() task.
#
# If you are new to Celery, I recommend reading the following tutorial:
# http://docs.celeryproject.org/en/latest/getting-started/first-steps-with-celery.html
#

from celery import Celery

# Create a main application that uses an AMQP queue as the broker.
# http://docs.celeryproject.org/en/latest/userguide/application.html
app = Celery('tasks', broker='pyamqp://guest:guest@localhost:5672//')

# Register a task.
# http://docs.celeryproject.org/en/latest/userguide/tasks.html
@app.task(ignore_result=True)
def hello(name, age):
    print('Hello {}. You are {} years old.'.format(name, age))
