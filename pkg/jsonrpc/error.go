package jsonrpc

import (
	"io"
)

type ErrorCode int

const (
	ErrParse          ErrorCode = -32700
	ErrInvalidRequest ErrorCode = -32600
	ErrMethodNotFound ErrorCode = -32601
	ErrInvalidParams  ErrorCode = -32602
	ErrInternalError  ErrorCode = -32603

	// Custom
	ErrRequestError ErrorCode = -32000
)

type respError struct {
	Code    ErrorCode `json:"code"`
	Message string    `json:"message"`
}

func respondError(w io.Writer, id any, code ErrorCode, err error) error {
	respond(w, response{
		Jsonrpc: "2.0",
		ID:      id,
		Error: &respError{
			Code:    code,
			Message: err.Error(),
		},
	})
	return err
}
