package V2

import (
	"fmt"
	rb "methods/relationrpc"
	"runtime/debug"
)

func Parse(in *rb.ParseRequest) (*rb.ParseRequest, error) {
	defer func() {
		if r := recover(); r != nil {
			err := fmt.Errorf("Parse error: %v\n%#v", r, debug.Stack())
		}
	}()
}
