package mihomo

import (
	"errors"
	"fmt"

	"github.com/metacubex/mihomo/constant"
	"github.com/metacubex/mihomo/listener"
	LC "github.com/metacubex/mihomo/listener/config"
	"github.com/metacubex/mihomo/listener/tproxy"
	"github.com/metacubex/mihomo/tunnel"
	"github.com/metacubex/mihomo/tunnel/statistic"
)


func Stop() error {
	mu.Lock()
	defer mu.Unlock()

	if !started {
		return nil
	}

	tunnel.OnSuspend() // Запрещаем обработку новых подключений

	// Закрываем стандартные слушатели,
 	// в mihomo это делается передавая нулевой порт
	listener.ReCreateHTTP(0, tunnel.Tunnel) // Закрываем HTTP
        listener.ReCreateSocks(0, tunnel.Tunnel) // Закрываем Socks
        listener.ReCreateRedir(0, tunnel.Tunnel) //Закрываем redir
        listener.ReCreateTProxy(0, tunnel.Tunnel) // Закрываем TProxy
        listener.ReCreateMixed(0, tunnel.Tunnel) // Закрываем Mixed режим

	// Закрываем доп слушатели, тут указатели уже на конфиги в виде string,
 // потому передаем просто пустую строку
	listener.ReCreateShadowSocks("", tunnel.Tunnel)
	listener.ReCreateVmess("", tunnel.Tunnel)
	listener.ReCreateTuic(LC.TuicServer{}, tunnel.Tunnel)

	// Удаляем доп inbound listeners
        listener.PatchInboundListeners(map[string]constant.InboundListener{}, tunnel.Tunnel, true)

	// Удаляем кастомные TCP/UDP тунели
        listener.PatchTunnel(nil, tunnel.Tunnel)


	// Закрываем tun
        tunConfig := listener.GetTunConf() // Вызываем конфу с tun из ram
	if tunConfig.Enable {
		tunConfig.Enable = false
		listener.ReCreateTun(tunConfig, tunnel.Tunnel)
	}

	// Очищаем системные правила
        tproxy.CleanupTProxyIPTables()

	// Массив в который мы будем собирать ошибки,
 // если установленные соединения закрыть не удалось
	var closeErrors []error

	// Перебираем подключения из менеджера mihomo, выключаем поочередно и собираем ошибки
	statistic.DefaultManager.Range(func (c statistic.Tracker) bool {
		if err := c.Close(); err != nil {
			closeErrors = append(closeErrors, fmt.Errorf("close connection: %s, %w", c.ID(), err))
		}

		return true
	})

	started = false

	return errors.Join(closeErrors...)

}
