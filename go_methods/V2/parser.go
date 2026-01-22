package V2

import (
	"fmt"
	rb "methods/relationrpc"
	"os"
	"runtime/debug"
)

func Parse(in *rb.ParseRequest) (resp *rb.ParseResponse, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("Parse error: %v\n%s", r, debug.Stack())
			resp = &rb.ParseResponse{
				ResponceFlag: rb.ResponseFlag_FAILED,
				Message:      err.Error(),
			}
		}
	}()

	content := in.Content

	if in.TempPath != "" {
		data, err := os.ReadFile(in.TempPath)
		if err != nil {
			return &rb.ParseResponse{
				ResponceFlag: rb.ResponseFlag_FAILED,
				Message:      err.Error(),
			}, err
		}
		content = string(data)
	}

}
