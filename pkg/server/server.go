package server

import (
	"context"
	"errors"
	"fmt"
	"net/http"

	"golang.org/x/exp/slog"

	"nfsense.net/nfsense/pkg/definitions"
	"nfsense.net/nfsense/pkg/jsonrpc"
)

var server http.Server
var mux = http.NewServeMux()
var apiHandler *jsonrpc.Handler

func StartWebserver(conf *definitions.Config, _apiHandler *jsonrpc.Handler) {
	server.Addr = ":8080"
	server.Handler = mux
	apiHandler = _apiHandler

	// Routing
	mux.HandleFunc("/login", HandleLogin)
	mux.HandleFunc("/logout", HandleLogout)
	mux.HandleFunc("/session", HandleSession)
	mux.HandleFunc("/api", HandleAPI)
	mux.HandleFunc("/ws/api", HandleWebsocketAPI)
	mux.HandleFunc("/", HandleWebinterface)

	go func() {
		if err := server.ListenAndServe(); !errors.Is(err, http.ErrServerClosed) {
			slog.Error("Webserver error", err)
		}
		slog.Info("Webserver Stopped")
	}()
}

func ShutdownWebserver(ctx context.Context) error {
	err := server.Shutdown(ctx)
	if err != nil {
		return fmt.Errorf("Shutting down: %w", err)
	}
	return nil
}
