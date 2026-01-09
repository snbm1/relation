package main

import "C"
import (
	_ "fmt"
	V2 "methods/V2"
	_ "methods/manager"

	"github.com/sagernet/sing-box/log"
)

//export setup
func setup(basicPath, workingPath, tempPath *C.char, statusPort C.longlong, debug, enableservices bool) *C.char {
	err := V2.Setup(C.GoString(basicPath), C.GoString(workingPath), C.GoString(tempPath), int64(statusPort), debug, enableservices)
	return errorOrNot(err)
}

func errorOrNot(err error) *C.char {
	if err == nil {
		return C.CString("")
	}
	log.Error(err.Error())
	return C.CString(err.Error())
}
