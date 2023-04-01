package main

import (
	"context"
	"flag"
	"os"
	"os/signal"
	"syscall"
	"time"

	"golang.org/x/exp/slog"
	configAPI "nfsense.net/nfsense/internal/api/config"
	"nfsense.net/nfsense/internal/api/firewall"
	"nfsense.net/nfsense/internal/api/network"
	"nfsense.net/nfsense/internal/api/object"
	"nfsense.net/nfsense/internal/config"
	"nfsense.net/nfsense/internal/jsonrpc"
	"nfsense.net/nfsense/internal/server"
)

func main() {
	applyPtr := flag.Bool("apply", false, "apply config and stop")
	flag.Parse()

	slog.Info("Starting...")

	configManager := config.CreateConfigManager()

	err := configManager.LoadCurrentConfigFromDisk()
	if err != nil {
		slog.Error("Loading Current Config", err)
		os.Exit(1)
	}

	slog.Info("Config Loaded")

	err = configManager.LoadPendingConfigFromDisk()
	if err != nil {
		slog.Error("Loading Pending Config", err)
		err = configManager.DiscardPendingConfig()
		if err != nil {
			slog.Error("Discarding Pending Config", err)
			os.Exit(1)
		}
	}

	if *applyPtr {
		slog.Info("Applying Config...")
		err := configManager.ApplyPendingChanges()
		if err != nil {
			slog.Error("Applying Pending Config", err)
			os.Exit(1)
		}
		slog.Info("Config Applied, Exiting...")
		return
	}

	slog.Info("Setup API...")
	apiHandler := jsonrpc.NewHandler(100 << 20)
	RegisterAPIMethods(apiHandler, configManager)

	slog.Info("Starting Webserver...")
	server.StartWebserver(configManager, apiHandler)

	slog.Info("Ready.")

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

func RegisterAPIMethods(apiHandler *jsonrpc.Handler, configManager *config.ConfigManager) {
	apiHandler.Register("Config", &configAPI.Config{ConfigManager: configManager})
	apiHandler.Register("Firewall", &firewall.Firewall{ConfigManager: configManager})
	apiHandler.Register("Network", &network.Network{ConfigManager: configManager})
	apiHandler.Register("Object", &object.Object{ConfigManager: configManager})
}
