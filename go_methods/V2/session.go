package V2

import (
	"fmt"
	"os"
	"runtime/debug"
	"sync"

	"github.com/sagernet/sing-box/experimental/libbox"
)

var (
	Box *libbox.BoxService
	mut sync.Mutex
)

func Start(configPath string, Memorylimit bool) (err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("Panic in Start func: %v\n%s", r, debug.Stack())
		}
	}()

	mut.Lock()
	defer mut.Unlock()

	if Box != nil {
		StopUnlocked()
		Box = nil
	}

	libbox.SetMemoryLimit(Memorylimit)
	file, err := os.ReadFile(configPath)
	if err != nil {
		return fmt.Errorf("Can't read the file with config: %w", err)

	}

	content := string(file)
	parsedContent, err := readOptions(content)

	if err != nil {
		return fmt.Errorf("Can't parse config correctly: %w", err)
	}

	instance, err := NewService(parsedContent)
	if err != nil {
		return fmt.Errorf("Can't start New Service successfully: %w", err)
	}

	err = instance.Start()
	if err != nil {
		return fmt.Errorf("Can't start instance successfully: %w", err)
	}

	Box = instance
	return nil
}

func StopUnlocked() (err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("Panic in StopUnlocked function: %v\n%s", r, debug.Stack())
		}
	}()

	if Box == nil {
		return fmt.Errorf("Sing-Box is not running")
	}

	err = Box.Close()
	if err != nil {
		return fmt.Errorf("Error while stopping service: %w", err.Error())
	}

	Box = nil
	return
}

func Stop() (err error) {

	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("Panic in StopFunc: %v\n%s", r, debug.Stack())
		}
	}()

	mut.Lock()
	defer mut.Unlock()

	if Box == nil {
		return fmt.Errorf("Sing-Box is not running")
	}

	err = Box.Close()
	if err != nil {
		return fmt.Errorf("Error while stopping service: %w", err.Error())
	}

	Box = nil
	return
}
