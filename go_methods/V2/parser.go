package V2

import (
	"fmt"
	"methods/config"
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

func Parse(content string, tempPath string, debugger bool) (string, error) {
	defer func() {
		if r := recover(); r != nil {
			panic(fmt.Errorf("Parse panic: %v\n%s", r, debug.Stack()))
		}
	}()

	if content == "" && tempPath != "" {
		data, err := os.ReadFile(tempPath)
		if err != nil {
			return "", err
		}
		content = string(data)
	}

	if content == "" {
		return "", fmt.Errorf("empty config")
	}

	parsed, err := config.ConfigParser(content)
	if err != nil {
		return "", err
	}

	return string(parsed), nil

}
