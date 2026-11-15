package main

/*
#include <stdlib.h>
*/
import "C"

import (
	_ "fmt"
	M "methods/go_methods/mihomo"
)

//export start
func start(configPath *C.char) *C.char {
	if configPath == nil {
		return C.CString("config is empty!")
	}

	err := M.Start(C.GoString(configPath))

	if err != nil {
		return C.CString(err.Error())
	}

	return C.CString("")
}
