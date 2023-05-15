package server

import (
	"context"
	"errors"
	"fmt"
	"net/http"

	"golang.org/x/exp/slog"

	"nfsense.net/nfsense/internal/config"
	"nfsense.net/nfsense/internal/jsonrpc"
	"nfsense.net/nfsense/internal/session"
)

var server http.Server
var mux = http.NewServeMux()
var apiHandler *jsonrpc.Handler
var stopCleanup chan struct{}
var configManager *config.ConfigManager

func StartWebserver(_configManager *config.ConfigManager, _apiHandler *jsonrpc.Handler) {
	server.Addr = ":8080"
	server.Handler = mux
	apiHandler = _apiHandler
	configManager = _configManager

	// Routing
	mux.HandleFunc("/login", HandleLogin)
	mux.HandleFunc("/logout", HandleLogout)
	mux.HandleFunc("/session", HandleSession)
	mux.HandleFunc("/api", HandleAPI)
	mux.HandleFunc("/ws/api", HandleWebsocketAPI)
	mux.HandleFunc("/", HandleWebinterface)

	stopCleanup = make(chan struct{})

	go session.CleanupSessions(stopCleanup)

	go func() {
		if err := server.ListenAndServe(); !errors.Is(err, http.ErrServerClosed) {
			slog.Error("Webserver error", "err", err)
		}
		slog.Info("Webserver Stopped")
	}()
}

func ShutdownWebserver(ctx context.Context) error {
	stopCleanup <- struct{}{}
	err := server.Shutdown(ctx)
	if err != nil {
		return fmt.Errorf("Shutting down: %w", err)
	}
	return nil
}
