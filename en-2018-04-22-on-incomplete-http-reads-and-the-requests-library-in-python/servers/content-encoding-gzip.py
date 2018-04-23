#!/usr/bin/env python3
#
# An HTTP server that returns fewer bytes in the body of a gzipped response than
# stated in the Content-Length header.
#

import socketserver

class MyTCPHandler(socketserver.BaseRequestHandler):
    def handle(self):
        data = self.request.recv(1024)
        print(data.decode())
        self.request.sendall(
            b'HTTP/1.1 200 OK\r\n'
            b'Content-Encoding: gzip\r\n'
            b'Content-Length: 30\r\n'
            b'\r\n'
            # gzipped "hello" (25 bytes):
            b'\x1f\x8b\x08\x00\xc9)\xdcZ\x00\x03\xcbH\xcd\xc9\xc9\x07\x00\x86\xa6\x106\x05\x00\x00\x00'
        )

class MyTCPSever(socketserver.TCPServer):
    allow_reuse_address = True

with MyTCPSever(('localhost', 8080), MyTCPHandler) as server:
    server.serve_forever()
