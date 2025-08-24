#include <winsock2.h>
#include <ws2tcpip.h>
#include <iostream>
#include <string>
#include <ctime>

#pragma comment(lib, "ws2_32.lib")

int main(int argc, char* argv[]) {
    // Default port
    int port = 8080;
    
    // Parse command line arguments
    if (argc > 1) {
        try {
            port = std::stoi(argv[1]);
            if (port <= 0 || port > 65535) {
                std::cerr << "Error: Port must be between 1 and 65535" << std::endl;
                return 1;
            }
        } catch (const std::exception& e) {
            std::cerr << "Error: Invalid port number '" << argv[1] << "'" << std::endl;
            std::cerr << "Usage: " << argv[0] << " [port]" << std::endl;
            std::cerr << "Example: " << argv[0] << " 3000" << std::endl;
            return 1;
        }
    }

    WSADATA wsa;
    SOCKET server_fd, client_fd;
    sockaddr_in server, client;
    int c;
    char buffer[1024];

    // Initialize Winsock
    if (WSAStartup(MAKEWORD(2,2), &wsa) != 0) {
        std::cerr << "WSAStartup failed" << std::endl;
        return 1;
    }

    // Create socket
    server_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server_fd == INVALID_SOCKET) {
        std::cerr << "Socket failed" << std::endl;
        WSACleanup();
        return 1;
    }

    // Setup address
    server.sin_family = AF_INET;
    server.sin_addr.s_addr = INADDR_ANY;
    server.sin_port = htons(port);

    // Bind
    if (bind(server_fd, (struct sockaddr*)&server, sizeof(server)) == SOCKET_ERROR) {
        int error = WSAGetLastError();
        if (error == WSAEADDRINUSE) {
            std::cerr << "Error: Port " << port << " is already in use" << std::endl;
        } else {
            std::cerr << "Bind failed with error: " << error << std::endl;
        }
        closesocket(server_fd);
        WSACleanup();
        return 1;
    }

    // Listen
    if (listen(server_fd, 3) == SOCKET_ERROR) {
        std::cerr << "Listen failed" << std::endl;
        closesocket(server_fd);
        WSACleanup();
        return 1;
    }

    std::cout << "Server running on http://localhost:" << port << std::endl;
    std::cout << "Press Ctrl+C to stop the server" << std::endl;

    c = sizeof(sockaddr_in);
    while (true) {
        // Accept
        client_fd = accept(server_fd, (struct sockaddr*)&client, &c);
        if (client_fd == INVALID_SOCKET) {
            std::cerr << "Accept failed" << std::endl;
            continue;
        }

        // Receive request
        int valread = recv(client_fd, buffer, sizeof(buffer) - 1, 0);
        if (valread > 0) {
            buffer[valread] = '\0';
            std::cout << "Request received from client:" << std::endl;
            std::cout << "----------------------------------------" << std::endl;
            std::cout << buffer << std::endl;
            std::cout << "----------------------------------------" << std::endl;
        }

        // Create response with port information
        std::string response_body = 
            "<html><body>"
            "<h1>Hello from C++ HTTP Server (Windows)</h1>"
            "<p>Server running on port: " + std::to_string(port) + "</p>"
            "<p>Time: " + std::to_string(time(nullptr)) + "</p>"
            "</body></html>";

        std::string response =
            "HTTP/1.1 200 OK\r\n"
            "Content-Type: text/html\r\n"
            "Content-Length: " + std::to_string(response_body.length()) + "\r\n"
            "Connection: close\r\n"
            "\r\n" + response_body;

        send(client_fd, response.c_str(), response.length(), 0);

        closesocket(client_fd);
    }

    closesocket(server_fd);
    WSACleanup();
    return 0;
}