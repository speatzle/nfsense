package server

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/pkg/jsonrpc"
)

var apiHandler jsonrpc.Handler

func init() {
	apiHandler = jsonrpc.NewHandler(100 << 20)
	apiHandler.Register("test", Ping{})
}

type Ping struct {
}

type PingRequest struct {
	Msg string `json:"msg"`
}

type PingResponse struct {
	Msg string `json:"msg"`
}

func (p Ping) Ping(ctx context.Context, req PingRequest) (*PingResponse, error) {
	if req.Msg == "" {
		return nil, fmt.Errorf("Message is empty")
	}
	return &PingResponse{
		Msg: req.Msg,
	}, nil
}
