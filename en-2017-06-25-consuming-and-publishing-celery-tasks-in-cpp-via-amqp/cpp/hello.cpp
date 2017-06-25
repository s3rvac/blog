//
// Sends a request to call hello() from within a worker.
//
// Uses SimpleAmqpClient (https://github.com/alanxz/SimpleAmqpClient) to
// connect to RabbitMQ and nlohmann/json (https://github.com/nlohmann/json) to
// create JSON.
//

#include <iostream>
#include <string>

// Access to https://github.com/alanxz/SimpleAmqpClient
#include <SimpleAmqpClient/SimpleAmqpClient.h>

// Access to https://github.com/nlohmann/json
#include <json.hpp>

// A convenience type alias.
using json = nlohmann::json;

int main(int argc, char** argv) {
	// Two arguments are required: name (string) and age (int).
	if (argc != 3) {
		std::cout << "usage: " << argv[0] << " NAME AGE\n";
		return 1;
	}

	// Parse the arguments.
	auto name = std::string(argv[1]);
	auto age = std::stoi(argv[2]);

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

	// Create a body of the message.
	//
	// This assumes that Celery uses JSON as a way of serializing and
	// de-serializing messages, which is the default.
	auto body = json{
		{name, age},    // args
		json::object(), // kwargs
		json::object()  // other data
	};

	// Create a message with the above body, serialized into a string.
	auto msg = AmqpClient::BasicMessage::Create(body.dump());

	// As said above, Celery by default uses JSON to serialize and de-serialize
	// message bodies.
	msg->ContentType("application/json");

	// Assume UTF-8.
	msg->ContentEncoding("utf-8");

	// Celery requires two headers: id and task. The former is a UUID string.
	// The latter has to be the name of the task, as registered in the Python
	// part.
	//
	// In a real world scenario, you would probably want generate a random
	// UUID ;-).
	msg->HeaderTable({
		{"id", "3149beef-be66-4b0e-ba47-2fc46e4edac3"},
		{"task", "tasks.hello"}
	});

	// Send the message to the 'celery' exchange (default) with the 'celery'
	// routing key (default). This effectively sends the message to the
	// 'celery' queue because 'celery' is a direct exchange
	// (https://www.rabbitmq.com/tutorials/amqp-concepts.html).
	channel->BasicPublish("celery", "celery", msg);

	return 0;
}
