package V2

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"os"
	"runtime"
	runtimeDebug "runtime/debug"
	"time"

	SingBox "github.com/sagernet/sing-box"
	box "github.com/sagernet/sing-box"
	"github.com/sagernet/sing-box/common/urltest"
	"github.com/sagernet/sing-box/experimental/libbox"
	"github.com/sagernet/sing-box/log"
	"github.com/sagernet/sing-box/option"
	Ex "github.com/sagernet/sing/common/exceptions"
	"github.com/sagernet/sing/service"
	"github.com/sagernet/sing/service/filemanager"
)

var (
	thisworkingPath           string
	thisTempPath              string
	UserID                    int
	GroupID                   int
	thisstatusPropagationPort int64
	coreLogFactory            log.Factory //Исправить, дописать файл который будет его импортировать!!!
)

type NewOption struct {
	option.Options
}

func (o *NewOption) UnmarshalJSON(content []byte) error {
	decoder := json.NewDecoder(bytes.NewReader(content))
	decoder.DisallowUnknownFields()
	err := decoder.Decode(&o.Options)
	if err != nil {
		return err
	}
	o.RawMessage = content
	return nil
}

func Setup(basicPath, workingPath, tempPath string, statusPort int64, debug bool) error {
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

	return nil
}

func NewService(options option.Options) (*box.Box, error) {
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
	return instance, nil
}

func readOptions(configContent string) (option.Options, error) {
	var options NewOption
	err := options.UnmarshalJSON([]byte(configContent))
	if err != nil {
		return option.Options{}, Ex.Cause(err, "error decoding config")
	}
	return options.Options, nil
}
