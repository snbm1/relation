package V2

import (
	"fmt"

	"github.com/sagernet/sing-box/experimental/libbox"
)

func UrlTest(tag string) error {
	err := libbox.NewStandaloneCommandClient().URLTest(tag)
	if err != nil {
		return fmt.Errorf("error in Urltesting process: %w", err)
	}

	return nil
}
