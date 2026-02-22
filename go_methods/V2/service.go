package V2

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"os"
	"time"

	box "github.com/sagernet/sing-box"
	"github.com/sagernet/sing-box/experimental/libbox"
	"github.com/sagernet/sing-box/include"
	"github.com/sagernet/sing-box/log"
	"github.com/sagernet/sing-box/option"
	Ex "github.com/sagernet/sing/common/exceptions"
	sbjson "github.com/sagernet/sing/common/json"
)

var (
	thisworkingPath           string
	thisTempPath              string
	UserID                    int
	GroupID                   int
	thisstatusPropagationPort int64
	coreLogFactory            log.Factory
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
	var options libbox.SetupOptions = libbox.SetupOptions{BasePath: basicPath, WorkingPath: workingPath, TempPath: tempPath}
	libbox.Setup(&options)
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

func NewService(ctx context.Context, opts option.Options) (*box.Box, context.CancelFunc, error) {
	ctx = include.Context(ctx)

	runCtx, cancel := context.WithCancel(ctx)
	instance, err := box.New(box.Options{
		Context: runCtx,
		Options: opts,
	})
	if err != nil {
		cancel()
		return nil, nil, err
	}
	return instance, cancel, nil
}

func readOptions(ctx context.Context, configContent []byte) (option.Options, context.Context, error) {
	ctx = include.Context(ctx)

	opts, err := sbjson.UnmarshalExtendedContext[option.Options](ctx, configContent)
	if err != nil {
		return option.Options{}, ctx, err
	}
	return opts, ctx, nil
}
