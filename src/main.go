package main

import (
	"fmt"
	"log"
	"net"
)

var logger = log.Default()

func handleClient(conn net.Conn) {
	defer conn.Close()

	buffer := make([]byte, 1024)

	for {
		n, err := conn.Read(buffer)

		if err != nil {
			logger.Fatalln("Error occurred while trying to read from buffer.")
		}

		fmt.Printf("Received: %s\n", buffer[:n])
	}
}

func sendTcpPackage(conn net.Conn) {
	data := []byte("What is an API?")

	_, err := conn.Write(data)

	if err != nil {
		logger.Println("Error occurred trying to send TCP package")
		return
	}
}

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:7878")

	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	sendTcpPackage(conn)
}
