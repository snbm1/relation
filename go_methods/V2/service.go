package V2

import (
	"context"
	"io"
	"methods/manager"
	"os"
	"runtime"
	runtimeDebug "runtime/debug"
	"time"

	SingBox "github.com/sagernet/sing-box"
	"github.com/sagernet/sing-box/common/urltest"
	"github.com/sagernet/sing-box/experimental/libbox"
	"github.com/sagernet/sing-box/log"
	"github.com/sagernet/sing-box/option"
	Ex "github.com/sagernet/sing/common/exceptions"
	"github.com/sagernet/sing/service"
	"github.com/sagernet/sing/service/filemanager"
	"github.com/sagernet/sing/service/pause"
)

var (
	thisworkingPath           string
	thisTempPath              string
	UserID                    int
	GroupID                   int
	thisstatusPropagationPort int64
	coreLogFactory            log.Factory //Исправить, дописать файл который будет его импортировать!!!
)

func InitService() error {
	return manager.StartServices()
}

func Setup(basicPath, workingPath, tempPath string, statusPort int64, debug, enableservices bool) error {
	thisstatusPropagationPort = int64(statusPort)
	tcpConn := runtime.GOOS == "windows"
	libbox.Setup(basicPath, workingPath, tempPath, tcpConn)
	thisworkingPath = workingPath
	thisTempPath = tempPath
	os.Chdir(thisworkingPath)
	UserID = os.Getuid()
	GroupID = os.Getgid()

	var defaultWriter io.Writer
	if !debug {
		defaultWriter = io.Discard
	}

	factory, err := log.New(
		log.Options{
			DefaultWriter: defaultWriter,
			BaseTime:      time.Now(),
			Observable:    true,
		})
	coreLogFactory = factory

	if err != nil {
		return Ex.Cause(err, "error creating logger")
	}

	if enableservices {
		return InitService()
	}

	return nil
}

func NewService(options option.Options) (*libbox.BoxService, error) {
	runtimeDebug.FreeOSMemory()
	ctx, cancel := context.WithCancel(context.Background())
	ctx = filemanager.WithDefault(ctx, thisworkingPath, thisTempPath, UserID, GroupID)
	urlTestHistoryStorage := urltest.NewHistoryStorage()
	ctx = service.ContextWithPtr(ctx, urlTestHistoryStorage)
	instance, err := SingBox.New(SingBox.Options{
		Context: ctx,
		Options: options,
	})

	if err != nil {
		cancel()
		return nil, Ex.Cause(err, "error creating sing-box service")
	}

	runtimeDebug.FreeOSMemory()
	service := libbox.NewBoxService(
		ctx,
		cancel,
		instance,
		service.FromContext[pause.Manager](ctx),
		urlTestHistoryStorage,
	)
	return &service, nil
}

func readOptions(configContent string) (option.Options, error) {
	var options option.Options
	err := options.UnmarshalJSON([]byte(configContent))
	if err != nil {
		return option.Options{}, Ex.Cause(err, "error decoding config")
	}
	return options, nil
}
