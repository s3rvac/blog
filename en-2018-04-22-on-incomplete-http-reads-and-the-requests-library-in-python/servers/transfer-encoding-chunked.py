#!/usr/bin/env python3
#
# An HTTP server that returns less bytes in the body of the response than
# stated in the chunk size when using "Transfer-Encoding: Chunked".
#

import socketserver

class MyTCPHandler(socketserver.BaseRequestHandler):
    def handle(self):
        data = self.request.recv(1024)
        print(data.decode())
        self.request.sendall(
            b'HTTP/1.1 200 OK\r\n'
            b'Transfer-Encoding: chunked\r\n'
            b'\r\n'
            b'a\r\n'  # size of the chunk (0xa = 10)
            b'123456'
        )

class MyTCPSever(socketserver.TCPServer):
    allow_reuse_address = True

with MyTCPSever(('localhost', 8080), MyTCPHandler) as server:
    server.serve_forever()
