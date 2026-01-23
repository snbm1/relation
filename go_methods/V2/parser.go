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

	if content == "" && in.TempPath != "" {
		data, readErr := os.ReadFile(in.TempPath)
		if readErr != nil {
			return &rb.ParseResponse{
				ResponceFlag: rb.ResponseFlag_FAILED,
				Message:      readErr.Error(),
			}, readErr
		}
		content = string(data)
	}

	if content == "" {
		err = fmt.Errorf("empty config")
		return &rb.ParseResponse{
			ResponceFlag: rb.ResponseFlag_FAILED,
			Message:      err.Error(),
		}, err
	}

	parsed, parseErr := config.ConfigParser(content)
	if parseErr != nil {
		return &rb.ParseResponse{
			ResponceFlag: rb.ResponseFlag_FAILED,
			Message:      parseErr.Error(),
		}, parseErr
	}

	return &rb.ParseResponse{
		ResponceFlag: rb.ResponseFlag_OK,
		Content:      string(parsed),
		Message:      "",
	}, nil

}
