package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"log"
	"net"
	"os"
)

var logger = log.Default()
var headerRead = false

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func handleClient(conn net.Conn) {
	defer conn.Close()

  var buffer bytes.Buffer
	tmpBuffer := make([]byte, 1024)
 
  for {
    n, err := conn.Read(tmpBuffer)
    if n > 0 {
      buffer.Write(tmpBuffer[:n])
    }

    // Check for EOF (Disconnected client)
    if err != nil {
      if err == io.EOF {
        log.Println("Client disconnected.")
        break
      }
      log.Fatalf("Error occurred while trying to read from buffer: %v\n", err)
    }

    bufBytes := buffer.Bytes()
    
    if !headerRead {
      for i := 0; i < len(bufBytes)-1; i++ {
        if bufBytes[i] == '\n' && bufBytes[i+1] == '\n' {
          fmt.Print("Found new Line!")
          break
        }
      }
      headerRead = true
    } 

    errIo := logToFile(tmpBuffer[:n])

    check(errIo)

    if err != nil {
      logger.Fatalln("Error occurred while trying to read from buffer.")
    }

    fmt.Printf("Received: %s\n", tmpBuffer[:n])

  }
}

func logToFile(data []byte) error {
	file, err := os.OpenFile("../logs/log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.Write(data)
	return err
}

func sendTcpPackage(conn net.Conn) {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter question: ")
	text, err := reader.ReadString('\n')
	data := []byte(text)
	if err != nil {
		panic(err)
	}
	_, err = conn.Write(data)

	if err != nil {
		logger.Println("Error occurred trying to send TCP package")
		return
	}

	handleClient(conn)
}

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:7878")

	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	sendTcpPackage(conn)
}
