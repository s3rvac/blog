#!/usr/bin/env python3
#
# An HTTP server that returns fewer bytes in the body of the response than
# stated in the Content-Length header.
#

import socketserver

class MyTCPHandler(socketserver.BaseRequestHandler):
    def handle(self):
        data = self.request.recv(1024)
        print(data.decode())
        self.request.sendall(
            b'HTTP/1.1 200 OK\r\n'
            b'Content-Length: 10\r\n'
            b'\r\n'
            b'123456'
        )

class MyTCPSever(socketserver.TCPServer):
    allow_reuse_address = True

with MyTCPSever(('localhost', 8080), MyTCPHandler) as server:
    server.serve_forever()
