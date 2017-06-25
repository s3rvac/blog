#
# A CMake build script for the C++ part.
#

cmake_minimum_required(VERSION 3.1)
project(cpp-part CXX C)
set(CMAKE_CXX_STANDARD 11)

# For SimpleAmqpClient.
find_package(Boost 1.47 REQUIRED system chrono)

include(ExternalProject)

# Put all external projects into the following directory.
set_directory_properties(PROPERTIES EP_PREFIX "${CMAKE_BINARY_DIR}/external")

# SimpleAmqpClient
ExternalProject_Add(simple-amqp-client
    URL "https://github.com/alanxz/SimpleAmqpClient/archive/v2.4.0.zip"
    URL_HASH SHA1=931e2aa78fc011f8d1ea312541df75b1d5edd559
    CMAKE_ARGS
        "-DCMAKE_BUILD_TYPE=Release"
        "-DCMAKE_C_COMPILER=${CMAKE_C_COMPILER}"
        "-DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}"
	# Wrap the download, configure and build steps in a script to log the
	# output.
	LOG_DOWNLOAD ON
    LOG_CONFIGURE ON
    LOG_BUILD ON
    # Disable the installation step.
    INSTALL_COMMAND ""
)
ExternalProject_Get_Property(simple-amqp-client source_dir)
ExternalProject_Get_Property(simple-amqp-client binary_dir)
set(SIMPLE_AMQP_CLIENT_INCLUDE_DIRS
	"${source_dir}/src"
	${Boost_INCLUDE_DIRS}
)
set(SIMPLE_AMQP_CLIENT_LIBRARY_DIR "${binary_dir}")
set(SIMPLE_AMQP_CLIENT_LIBRARIES
	"${SIMPLE_AMQP_CLIENT_LIBRARY_DIR}/${CMAKE_SHARED_LIBRARY_PREFIX}SimpleAmqpClient${CMAKE_SHARED_LIBRARY_SUFFIX}"
	${Boost_LIBRARIES}
)

# nlohmann/json
ExternalProject_Add(json
	URL https://github.com/nlohmann/json/archive/v2.1.1.zip
	URL_HASH SHA1=a89dcd91f05b2e11915c557e1e695707503977e4
    CMAKE_ARGS
        "-DCMAKE_BUILD_TYPE=Release"
        "-DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}"
        "-DBuildTests=OFF"
	# Wrap the download, configure and build steps in a script to log the
	# output.
	LOG_DOWNLOAD ON
	LOG_CONFIGURE ON
	LOG_BUILD ON
	# Disable the installation step.
	INSTALL_COMMAND ""
)
ExternalProject_Get_Property(json source_dir)
set(JSONCPP_INCLUDE_DIRS "${source_dir}/src")

# hello
add_executable(hello hello.cpp)
add_dependencies(hello
	simple-amqp-client
	json
)
target_include_directories(hello SYSTEM PRIVATE
	${SIMPLE_AMQP_CLIENT_INCLUDE_DIRS}
	${JSONCPP_INCLUDE_DIRS}
)
target_link_libraries(hello PRIVATE
	${SIMPLE_AMQP_CLIENT_LIBRARIES}
)

# worker
add_executable(worker worker.cpp)
add_dependencies(worker
	simple-amqp-client
	json
)
target_include_directories(worker SYSTEM PRIVATE
	${SIMPLE_AMQP_CLIENT_INCLUDE_DIRS}
	${JSONCPP_INCLUDE_DIRS}
)
target_link_libraries(worker PRIVATE
	${SIMPLE_AMQP_CLIENT_LIBRARIES}
)
