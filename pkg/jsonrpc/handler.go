package jsonrpc

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"reflect"
)

type Handler struct {
	methods map[string]method

	maxRequestSize int64
}

func NewHandler(maxRequestSize int64) Handler {
	return Handler{
		methods:        map[string]method{},
		maxRequestSize: maxRequestSize,
	}
}

func (h *Handler) HandleRequest(ctx context.Context, r io.Reader, w io.Writer) error {
	var req request
	bufferedRequest := new(bytes.Buffer)
	reqSize, err := bufferedRequest.ReadFrom(io.LimitReader(r, h.maxRequestSize+1))
	if err != nil {
		return respondError(w, "", ErrInternalError, fmt.Errorf("Reading Request: %w", err))
	}
	if reqSize > h.maxRequestSize {
		return respondError(w, "", ErrParse, fmt.Errorf("Request exceeds Max Request Size"))
	}

	dec := json.NewDecoder(bufferedRequest)
	dec.DisallowUnknownFields()
	err = dec.Decode(&req)
	if err != nil {
		return respondError(w, "", ErrParse, fmt.Errorf("Parsing Request: %w", err))
	}

	method, ok := h.methods[req.Method]
	if !ok {
		return respondError(w, "", ErrMethodNotFound, fmt.Errorf("Unknown Method %v", req.Method))
	}

	p := reflect.New(method.inType)
	paramPointer := p.Interface()

	dec = json.NewDecoder(bytes.NewReader(req.Params))
	dec.DisallowUnknownFields()
	err = dec.Decode(paramPointer)
	if err != nil {
		return respondError(w, "", ErrInvalidParams, fmt.Errorf("Parsing Request: %w", err))
	}

	params := make([]reflect.Value, 3)
	params[0] = method.subSystem
	params[1] = reflect.ValueOf(ctx)
	params[2] = reflect.ValueOf(paramPointer).Elem()
	res := method.handlerFunc.Call(params)
	result := res[0].Interface()

	if !res[1].IsNil() {
		reqerr := res[1].Interface().(error)
		return respondError(w, req.ID, 0, reqerr)
	}

	respondResult(w, req.ID, result)
	return nil
}
