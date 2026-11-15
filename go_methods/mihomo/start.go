package mihomo

import (
	"fmt"
	"os"
	"sync"

	"github.com/metacubex/mihomo/hub"
)

var (
	mu      sync.Mutex
	started bool
)

func Start(configPath string) error {
	mu.Lock()
	defer mu.Unlock()

	if started {
		return fmt.Errorf("mihomo already started")
	}

	configBytes, err := os.ReadFile(configPath)
	if err != nil {
		return fmt.Errorf("read config: %w", err)
	}

	if len(configBytes) == 0 {
		return fmt.Errorf("config file is empty")
	}

	if err := hub.Parse(configBytes); err != nil {
		return fmt.Errorf("mihomo start: %w", err)
	}

	started = true
	return nil
}
