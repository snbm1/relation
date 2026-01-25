package main

import "C"
import (
	_ "fmt"
	V2 "methods/V2"
	_ "methods/manager"
	rb "methods/relationrpc"

	"github.com/sagernet/sing-box/log"
)

//export setup
func setup(basicPath, workingPath, tempPath *C.char, statusPort C.longlong, debug, enableservices bool) *C.char {
	err := V2.Setup(C.GoString(basicPath), C.GoString(workingPath), C.GoString(tempPath), int64(statusPort), debug, enableservices)
	return errorOrNot(err)
}

//export parse
func parse(content *C.char, tempPath *C.char, debug bool) *C.char {
	res, err := V2.Parse(&rb.ParseRequest{
		Content:  C.GoString(content),
		TempPath: C.GoString(tempPath),
		Debug:    debug,
	})

	if err != nil {
		log.Error(err.Error())
		return C.CString(err.Error())
	}

	if res.ResponceFlag != rb.ResponseFlag_OK {
		return C.CString(res.Message)
	}

	return C.CString(res.Content)

}

//export start
func start(configPath *C.char, Memorylimit bool) *C.char {
	err := V2.Start(C.GoString(configPath), Memorylimit)
	return errorOrNot(err)
}

//export stop
func stop() *C.char {
	err := V2.Stop()
	return errorOrNot(err)
}

func errorOrNot(err error) *C.char {
	if err == nil {
		return C.CString("")
	}
	log.Error(err.Error())
	return C.CString(err.Error())
}
