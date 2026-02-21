package relationrpc

import (
	"context"
	"methods/go_methods/V2"
)

type CoreService struct {
	UnimplementedCoreServer
}

func (s *CoreService) Setup(ctx context.Context, req *SetupRequest) (*BaseResponse, error) {
	err := V2.Setup(
		req.BasePath,
		req.WorkingPath,
		req.TempPath,
		req.StatusPort,
		req.Debug,
	)

	if err != nil {
		return &BaseResponse{
			Ok:      false,
			Message: err.Error(),
		}, nil
	}

	return &BaseResponse{
		Ok:      true,
		Message: "",
	}, nil
}

func (s *CoreService) Parse(ctx context.Context, req *ParseRequest) (*ParseResponse, error) {
	resp, err := V2.Parse(req.Content, req.TempPath)

	if err != nil {
		return &ParseResponse{
			ResponseFlag: ResponseFlag_FAILED,
			Message:      err.Error(),
		}, nil
	}

	return &ParseResponse{
		ResponseFlag: ResponseFlag_OK,
		Content:      resp,
		Message:      "",
	}, nil
}

func (s *CoreService) Start(ctx context.Context, req *StartRequest) (*BaseResponse, error) {
	err := V2.Start(req.ConfigPath, req.MemoryLimit)

	if err != nil {
		return &BaseResponse{
			Ok:      false,
			Message: err.Error(),
		}, nil
	}

	return &BaseResponse{
		Ok:      true,
		Message: "",
	}, nil
}

func (s *CoreService) Stop(ctx context.Context, _ *Empty) (*BaseResponse, error) {
	err := V2.Stop()

	if err != nil {
		return &BaseResponse{
			Ok:      false,
			Message: err.Error(),
		}, nil
	}

	return &BaseResponse{
		Ok:      true,
		Message: "",
	}, nil
}

func (s *CoreService) Restart(ctx context.Context, req *StartRequest) (*BaseResponse, error) {
	err := V2.Restart(req.ConfigPath, req.MemoryLimit)

	if err != nil {
		return &BaseResponse{
			Ok:      false,
			Message: err.Error(),
		}, nil
	}

	return &BaseResponse{
		Ok:      true,
		Message: "",
	}, nil
}

func (s *CoreService) UrlTest(ctx context.Context, req *UrlTestRequest) (*BaseResponse, error) {
	err := V2.UrlTest(req.Tag)

	if err != nil {
		return &BaseResponse{
			Ok:      false,
			Message: err.Error(),
		}, nil
	}

	return &BaseResponse{
		Ok:      true,
		Message: "",
	}, nil
}
