package main

import (
	"context"
	"flag"
	"os"
	"os/signal"
	"syscall"
	"time"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/pkg/server"
)

func main() {
	applyPtr := flag.Bool("apply", false, "apply config and stop")
	flag.Parse()

	slog.Info("Starting...")

	conf, err := LoadConfiguration("config.json")
	if err != nil {
		slog.Error("Loading Config", err)
		return
	}

	slog.Info("Config Loaded", "config", conf)

	if *applyPtr {
		slog.Info("Applying Config...")
		err := apply(conf)
		if err != nil {
			slog.Error("Applying Config", err)
			return
		}
		slog.Info("Config Applied, Exiting...")
		return
	}

	slog.Info("Starting Webserver...")
	server.StartWebserver(conf)

	slog.Info("Ready")

	// Handle Exit Signal
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan
	slog.Info("Got Signal, Exiting...")

	shutdownCtx, shutdownRelease := context.WithTimeout(context.Background(), 10*time.Second)
	defer shutdownRelease()

	server.ShutdownWebserver(shutdownCtx)

	slog.Info("Done")
}
