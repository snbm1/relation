package V2

import (
	"context"
	"fmt"
	"os"
	"runtime/debug"
	"sync"
	"time"

	box "github.com/sagernet/sing-box"
	"github.com/sagernet/sing-box/experimental/libbox"
	"github.com/sagernet/sing-box/include"
)

var (
	Box       *box.Box
	boxCancel context.CancelFunc
	mut       sync.Mutex
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
		if stopErr := stopUnlocked(); stopErr != nil {
			return stopErr
		}
	}

	libbox.SetMemoryLimit(Memorylimit)

	debug.FreeOSMemory()

	return startServiceUnlocked(configPath)
}

func Restart(configPath string, Memorylimit bool) (err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("Panic in Restart func: %v\n%s", r, debug.Stack())
		}
	}()

	mut.Lock()
	defer mut.Unlock()

	if Box == nil {
		return fmt.Errorf("instance not found")
	}

	if stopErr := stopUnlocked(); stopErr != nil {
		return stopErr
	}

	time.Sleep(100 * time.Millisecond)

	libbox.SetMemoryLimit(Memorylimit)
	debug.FreeOSMemory()

	return startServiceUnlocked(configPath)
}

func Stop() (err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("Panic in Stop func: %v\n%s", r, debug.Stack())
		}
	}()

	mut.Lock()
	defer mut.Unlock()

	return stopUnlocked()
}

func startServiceUnlocked(configPath string) error {
	configBytes, err := os.ReadFile(configPath)
	if err != nil {
		return fmt.Errorf("Can't read the file with config: %w", err)
	}

	baseCtx := include.Context(context.Background())
	opts, ctx, err := readOptions(baseCtx, configBytes)

	if err != nil {
		return fmt.Errorf("Can't parse config correctly: %w", err)
	}

	instance, cancel, err := NewService(ctx, opts)

	if err != nil {
		cancel()
		return fmt.Errorf("Can't start New Service successfully: error creating sing-box instance: %w", err)
	}

	if err := instance.Start(); err != nil {
		cancel()
		return fmt.Errorf("Can't start instance successfully: %w", err)
	}

	Box = instance
	boxCancel = cancel
	return nil
}

func stopUnlocked() error {
	if Box == nil {
		return fmt.Errorf("Sing-Box is not running")
	}

	if boxCancel != nil {
		boxCancel()
		boxCancel = nil
	}

	err := Box.Close()
	Box = nil

	if err != nil {
		return fmt.Errorf("Error while stopping service: %w", err)
	}

	debug.FreeOSMemory()
	return nil
}
