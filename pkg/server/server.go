package server

import (
	"context"
	"errors"
	"fmt"
	"net/http"

	"golang.org/x/exp/slog"

	"nfsense.net/nfsense/pkg/definitions"
)

var server http.Server

func StartWebserver(conf *definitions.Config) {
	server.Addr = ":8080"
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
