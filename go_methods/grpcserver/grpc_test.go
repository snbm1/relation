package grpcserver

import (
	"net"
	"testing"
	"time"
)

func TestGrpc(t *testing.T) {
	addr := "127.0.0.1:50051"

	err := StartGrpcServer(addr)
	if err != nil {
		t.Fatalf("StartGrpcServer failed: %v", err)
	}

	conn, err := net.DialTimeout("tcp", addr, time.Second)
	if err != nil {
		t.Fatalf("gRPC server not listening: %v", err)
	}

	conn.Close()
}
