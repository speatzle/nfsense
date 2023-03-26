package server

import (
	"context"
	"fmt"
	"net/http"
	"runtime/debug"
	"time"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/session"
)

func HandleAPI(w http.ResponseWriter, r *http.Request) {
	slog.Info("Api Handler hit")
	_, s := session.GetSession(r)
	if s == nil {
		// Fallthrough after so that jsonrpc can still deliver a valid jsonrpc error
		w.WriteHeader(http.StatusUnauthorized)
	}

	defer func() {
		if r := recover(); r != nil {
			slog.Error("Recovered Panic Handling HTTP API Request", fmt.Errorf("%v", r), "stack", debug.Stack())
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}
	}()
	ctx, cancel := context.WithTimeout(context.WithValue(r.Context(), session.SessionKey, s), time.Second*10)
	defer cancel()

	err := apiHandler.HandleRequest(ctx, s, r.Body, w)
	if err != nil {
		slog.Error("Handling HTTP API Request", err)
	}
}
