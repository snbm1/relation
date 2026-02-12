package V2

import (
	"context"
	"fmt"
	"net/netip"
	"sync"

	"github.com/sagernet/sing-box/common/settings"
	"github.com/sagernet/sing/common/metadata"
)

var (
	systemProxy *settings.LinuxSystemProxy
	sysmut      sync.Mutex
)

func MakeSockAddr(host string, port int) metadata.Socksaddr {
	ip, err := netip.ParseAddr(host)
	if err == nil {
		return metadata.Socksaddr{
			Addr: ip,
			Port: uint16(port),
		}
	}

	return metadata.Socksaddr{
		Fqdn: host,
		Port: uint16(port),
	}
}

func EnableSystemProxy(host string, port int, supp_socks bool) error {
	sysmut.Lock()
	defer sysmut.Unlock()

	if systemProxy != nil && systemProxy.IsEnabled() {
		return nil
	}

	proxyAddr := MakeSockAddr(host, port)
	ctx := context.Background()

	Prox, err := settings.NewSystemProxy(ctx, proxyAddr, supp_socks)

	if err != nil {
		return fmt.Errorf("Error in creating NewSystemProxy: %w", err)
	}

	if err := Prox.Enable(); err != nil {
		return fmt.Errorf("Error in enabling SystemProxy: %w", err)
	}

	systemProxy = Prox
	return nil

}

func DisableSystemProxy() error {
	sysmut.Lock()
	defer sysmut.Unlock()

	if systemProxy == nil && !systemProxy.IsEnabled() {
		return nil
	}

	if systemProxy.IsEnabled() {
		err := systemProxy.Disable()
		if err != nil {
			return fmt.Errorf("Error in Disabling SystemProxy: %w", err)
		}
	}

	systemProxy = nil
	return nil
}
