package V2

import (
	"fmt"
	"runtime/debug"
	"sync"

	"github.com/sagernet/sing-box/experimental/libbox"
)

var (
	Box *libbox.BoxService
	mut sync.Mutex
)

func Stop() (err error) {

	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("Panic in StopFunc: %v\n%s", r, debug.Stack())
		}
	}()

	mut.Lock()
	defer mut.Unlock()

	if Box == nil {
		return fmt.Errorf("Sing-Box not running")
	}

	err = Box.Close()
	if err != nil {
		return fmt.Errorf("Error while stopping service: %v", err.Error())
	}

	Box = nil
	return
}
