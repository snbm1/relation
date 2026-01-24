package V2

import (
	"fmt"
	"methods/config"
	rb "methods/relationrpc"
	"os"
	"runtime/debug"
)

func ParseWriter(content string, filePath string) error {
	parsed, err := config.ConfigParser(content)
	if err != nil {
		return err
	}

	tmp := filePath + ".tmp"
	if err := os.WriteFile(tmp, parsed, 0o644); err != nil {
		return err
	}

	return os.Rename(tmp, filePath)
}

func Parse(in *rb.ParseRequest) (resp *rb.ParseResponse, err error) {
	resp = &rb.ParseResponse{
		ResponceFlag: rb.ResponseFlag_FAILED,
	}
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("Parse error: %v\n%s", r, debug.Stack())
			resp.Message = err.Error()
		}
	}()

	content := in.Content

	if content == "" && in.TempPath != "" {
		var data []byte
		data, err = os.ReadFile(in.TempPath)
		if err != nil {
			resp.Message = err.Error()
			return
		}
		content = string(data)
	}

	if content == "" {
		err = fmt.Errorf("empty config")
		resp.Message = err.Error()
		return
	}

	var parsed []byte
	parsed, err = config.ConfigParser(content)
	if err != nil {
		resp.Message = err.Error()
		return
	}

	resp.ResponceFlag = rb.ResponseFlag_OK
	resp.Content = string(parsed)
	resp.Message = ""

	return
}
