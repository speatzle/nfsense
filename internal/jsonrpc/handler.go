package jsonrpc

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"reflect"
	"runtime/debug"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/session"
)

type Handler struct {
	methods map[string]method

	maxRequestSize int64
}

func NewHandler(maxRequestSize int64) *Handler {
	return &Handler{
		methods:        map[string]method{},
		maxRequestSize: maxRequestSize,
	}
}

func (h *Handler) HandleRequest(ctx context.Context, s *session.Session, r io.Reader, w io.Writer) error {
	defer func() {
		if r := recover(); r != nil {
			slog.Error("Recovered Panic Handling JSONRPC Request", fmt.Errorf("%v", r), "stack", debug.Stack())
		}
	}()
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
		return respondError(w, "", ErrParse, fmt.Errorf("Decodeing Request: %w", err))
	}

	if req.Jsonrpc != "2.0" {
		return respondError(w, req.ID, ErrMethodNotFound, fmt.Errorf("Unsupported Jsonrpc version %v", req.Jsonrpc))
	}

	if s == nil {
		return respondError(w, req.ID, 401, fmt.Errorf("Unauthorized"))
	}

	method, ok := h.methods[req.Method]
	if !ok {
		return respondError(w, req.ID, ErrMethodNotFound, fmt.Errorf("Unknown Method %v", req.Method))
	}

	p := reflect.New(method.inType)
	paramPointer := p.Interface()

	if len(req.Params) != 0 {
		dec = json.NewDecoder(bytes.NewReader(req.Params))
		dec.DisallowUnknownFields()
		err = dec.Decode(paramPointer)
		if err != nil {
			return respondError(w, req.ID, ErrInvalidParams, fmt.Errorf("Decoding Parameters: %w", err))
		}
	}

	params := make([]reflect.Value, 3)
	params[0] = method.subSystem
	params[1] = reflect.ValueOf(ctx)
	params[2] = reflect.ValueOf(paramPointer).Elem()

	defer func() {
		if r := recover(); r != nil {
			slog.Error("Recovered Panic Executing API Method", fmt.Errorf("%v", r), "method", req.Method, "id", req.ID, "stack", debug.Stack())
		}
	}()
	res := method.handlerFunc.Call(params)
	result := res[0].Interface()

	if !res[1].IsNil() {
		reqerr := res[1].Interface().(error)
		slog.Error("API Method", reqerr, "method", req.Method, "id", req.ID)
		respondError(w, req.ID, ErrInternalError, reqerr)
	}

	respondResult(w, req.ID, result)
	return nil
}
