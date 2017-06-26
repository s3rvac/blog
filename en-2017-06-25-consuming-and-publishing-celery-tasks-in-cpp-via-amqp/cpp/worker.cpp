//
// Starts a C++ worker that can execute the hello() Celery task.
//
// Uses SimpleAmqpClient (https://github.com/alanxz/SimpleAmqpClient) to
// connect to RabbitMQ and nlohmann/json (https://github.com/nlohmann/json) to
// parse JSON.
//

#include <csignal>
#include <iostream>
#include <string>

// Access to https://github.com/alanxz/SimpleAmqpClient
#include <SimpleAmqpClient/SimpleAmqpClient.h>

// Access to https://github.com/nlohmann/json
#include <json.hpp>

// A convenience type alias.
using json = nlohmann::json;

namespace {

// A flag that allows us to gracefully stop the worker by sending a signal to
// it, just like with the Celery worker in Python.
// http://en.cppreference.com/w/cpp/utility/program/sig_atomic_t
volatile std::sig_atomic_t keep_running = 1;

// A signal handler. When it receives either SIGINT (sent when you press
// Ctrl-C) or SIGTERM, it signalizes to the worker that it should stop running.
//
// As a side note, `extern "C"` is needed because signal handlers are expected
// to have C linkage and, in general, only use the features from the common
// subset of C and C++. It is implementation-defined if a function with C++
// linkage can be used as a signal handler.
// http://en.cppreference.com/w/cpp/utility/program/signal#Notes
extern "C" void signal_handler(int signal) {
	if (signal == SIGINT || signal == SIGTERM) {
		keep_running = 0;
	}
}

}

int main(int argc, char** argv) {
	// The worker does not accept any arguments.
	if (argc != 1) {
		std::cout << "usage: " << argv[0] << '\n';
		return 1;
	}

	// Setup signal handling. When any of the below signals are sent to the
	// program, our signal handler gets called.
	// http://en.cppreference.com/w/cpp/utility/program/signal
	std::signal(SIGINT, signal_handler);
	std::signal(SIGTERM, signal_handler);

	// Create a connection to our AMQP server (RabbitMQ).
	//
	// Technically, in AMQP, a single connection can contain multiple channels,
	// where channels that can be thought of as "lightweight connections that
	// share a single TCP connection"
	// (https://www.rabbitmq.com/tutorials/amqp-concepts.html).
	// However, the used AMQP library only allows creation of channels, not
	// connections.
	auto channel = AmqpClient::Channel::Create(
		/*host*/"localhost",
		/*port*/5672,
		/*username*/"guest",
		/*password*/"guest",
		/*vhost*/"/"
	);

	// Start consuming messages from the 'celery' queue.
	//
	// This method has to be called before we start consuming messages from the
	// queue. We pass an empty 'consumer_tag', which causes the method to
	// generate a consumer tag for us. The returned tag identifies the consumer
	// and is used later when receiving messages.
	//
	// Setting 'no_ack' to false disables automatic acknowledgements. Instead,
	// we acknowledge them manually after we have successfully processed the
	// task. Otherwise, a message would be acknowledged right after it was
	// received, even if its execution later fails.
	//
	// Setting 'exclusive' to false allows other workers to consume messages
	// from the queue (we may not be the only worker running).
	//
	// Finally, 'message_prefetch_count' is the maximal number of
	// unacknowledged messages that the server will deliver to us. We want to
	// always receive just a single message (i.e. no buffering).
	auto consumer_tag = channel->BasicConsume(
		/*queue*/"celery",
		/*consumer_tag*/"",
		/*no_local*/true,
		/*no_ack*/false,
		/*exclusive*/false,
		/*message_prefetch_count*/1
	);
	try {
		// Keep trying to consume messages until the user signalizes we should
		// stop (either via Ctrl-C or by sending the SIGTERM signal to the
		// process).
		while (keep_running) {
			// Try the receive a message.
			//
			// To enable stopping of the worker, we use a 1-second timeout.
			// When no message is delivered within the timeout, we check
			// whether we should keep running, and if so, we repeat the
			// receiving.
			AmqpClient::Envelope::ptr_t envelope;
			auto message_delivered = channel->BasicConsumeMessage(
				consumer_tag,
				envelope,
				/*timeout*/1000/*ms*/
			);
			if (!message_delivered) {
				continue;
			}

			// Parse task arguments from the body of the message.
			//
			// Celery by default encodes messages via JSON. For a description
			// of the message format, see hello.cpp.
			auto message = envelope->Message();
			auto body = json::parse(message->Body());
			auto name = body[0][0].get<std::string>();
			auto age = body[0][1].get<int>();

			// Process the message in the same way it is processed in the
			// Python part (see python/tasks.py).
			std::cout << "Hello " << name << ". You are " << age << " years old.\n";

			// Acknowledge the message (we have successfully finished its
			// execution).
			channel->BasicAck(envelope);
		}
	} catch (...) {
		// Poor man's finally block. Cancel consuming from the queue (see the
		// note below this try-catch block) and re-throw the exception.
		channel->BasicCancel(consumer_tag);
		throw;
	}
	// When the worker ends, we have to cancel consuming from the queue, which
	// we started when we called channel->BasicConsume().
	channel->BasicCancel(consumer_tag);

	return 0;
}
