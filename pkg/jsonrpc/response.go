package jsonrpc

import (
	"encoding/json"
	"io"

	"golang.org/x/exp/slog"
)

type response struct {
	Jsonrpc string     `json:"jsonrpc"`
	Result  any        `json:"result,omitempty"`
	ID      any        `json:"id"`
	Error   *respError `json:"error,omitempty"`
}

func respond(w io.Writer, resp response) {
	err := json.NewEncoder(w).Encode(resp)
	if err != nil {
		slog.Warn("write response", err)
	}
}

func respondResult(w io.Writer, id, res any) {
	respond(w, response{
		Jsonrpc: "2.0",
		ID:      id,
		Result:  res,
	})
}
