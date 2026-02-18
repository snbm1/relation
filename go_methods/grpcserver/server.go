package grpcserver

import (
	"log"
	"methods/go_methods/relationrpc"
	"net"

	"google.golang.org/grpc"
)

func StartGrpcServer(listenAddress string) error {
	lis, err := net.Listen("tcp", listenAddress)
	if err != nil {
		log.Printf("failed to listen: %v", err)
		return err
	}

	grpcsrv := grpc.NewServer()

	relationrpc.RegisterCoreServer(grpcsrv, &relationrpc.CoreService{})
	log.Printf("gRPC server listening on %s", listenAddress)

	go func() {
		if err := grpcsrv.Serve(lis); err != nil {
			log.Printf("gRPC serve error: %v", err)
		}
	}()

	return nil

}
