package v2

import (
	"io"
	"os"
	"runtime"
	"time"

	"methods/manager"

	"github.com/sagernet/sing-box/experimental/libbox"
	"github.com/sagernet/sing-box/log"
	Ex "github.com/sagernet/sing/common/exceptions"
)

var (
	thisworkingPath           string
	thisTempPath              string
	thisUserID                int
	thisGroupID               int
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
	thisUserID = os.Getuid()
	thisGroupID = os.Getgid()

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
