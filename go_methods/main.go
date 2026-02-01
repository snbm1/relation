package main

import "C"
import (
	_ "fmt"
	V2 "methods/V2"
	"methods/grpcserver"
	_ "methods/manager"

	"github.com/sagernet/sing-box/log"
)

//export setup
func setup(basicPath, workingPath, tempPath *C.char, statusPort C.longlong, debug, enableservices bool) *C.char {
	err := V2.Setup(C.GoString(basicPath), C.GoString(workingPath), C.GoString(tempPath), int64(statusPort), debug, enableservices)
	return errorOrNot(err)
}

//export parse
func parse(content *C.char, tempPath *C.char) *C.char {
	res, err := V2.Parse(
		C.GoString(content),
		C.GoString(tempPath),
	)

	if err != nil {
		log.Error(err.Error())
		return C.CString(err.Error())
	}

	return C.CString(res)

}

//export start
func start(configPath *C.char, Memorylimit bool) *C.char {
	err := V2.Start(C.GoString(configPath), Memorylimit)
	return errorOrNot(err)
}

//export restart
func restart(configPath *C.char, Memorylimit bool) *C.char {
	err := V2.Restart(C.GoString(configPath), Memorylimit)
	return errorOrNot(err)
}

//export stop
func stop() *C.char {
	err := V2.Stop()
	return errorOrNot(err)
}

//export urlTest
func urlTest(tag *C.char) *C.char {
	err := V2.UrlTest(C.GoString(tag))

	return errorOrNot(err)
}

//export startCoreGrpcServer
func startCoreGrpcServer(listenAddress *C.char) *C.char {
	err := grpcserver.StartGrpcServer(C.GoString(listenAddress))
	return errorOrNot(err)
}

func errorOrNot(err error) *C.char {
	if err == nil {
		return C.CString("")
	}
	log.Error(err.Error())
	return C.CString(err.Error())
}

func main() {}
