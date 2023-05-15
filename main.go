package main

import (
	"context"
	"errors"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/godbus/dbus/v5"
	"golang.org/x/exp/slog"
	configAPI "nfsense.net/nfsense/internal/api/config"
	"nfsense.net/nfsense/internal/api/firewall"
	"nfsense.net/nfsense/internal/api/network"
	"nfsense.net/nfsense/internal/api/object"
	"nfsense.net/nfsense/internal/api/service"
	"nfsense.net/nfsense/internal/api/system"
	"nfsense.net/nfsense/internal/api/vpn"
	"nfsense.net/nfsense/internal/chrony"
	"nfsense.net/nfsense/internal/config"
	dhcp "nfsense.net/nfsense/internal/dhcp_server"
	"nfsense.net/nfsense/internal/jsonrpc"
	"nfsense.net/nfsense/internal/networkd"
	"nfsense.net/nfsense/internal/nftables"
	"nfsense.net/nfsense/internal/server"
	"nfsense.net/nfsense/internal/unbound"
)

func main() {
	slog.Info("Starting...")

	dbusConn, err := dbus.ConnectSystemBus()
	if err != nil {
		slog.Error("Connecting to DBus", err)
		// os.Exit(1)
	}
	defer dbusConn.Close()

	configManager := config.CreateConfigManager()
	RegisterApplyFunctions(configManager)

	// Check for Subcommand

	apply := false
	if len(os.Args) > 1 {
		switch os.Args[1] {
		case "apply":
			apply = true
		case "setup":
			setup(configManager, dbusConn)
			return
		}
	}

	err = configManager.LoadCurrentConfigFromDisk()
	if err != nil {
		slog.Error("Loading Current Config", err)
		os.Exit(1)
	}

	slog.Info("Config Loaded")

	err = configManager.LoadPendingConfigFromDisk()
	if err != nil {
		if !errors.Is(err, os.ErrNotExist) {
			slog.Error("Loading Pending Config", err)
		}
		err = configManager.DiscardPendingConfig()
		if err != nil {
			slog.Error("Discarding Pending Config", err)
			os.Exit(1)
		}
	}

	if apply {
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
	RegisterAPIMethods(apiHandler, configManager, dbusConn)

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

func RegisterAPIMethods(apiHandler *jsonrpc.Handler, configManager *config.ConfigManager, dbusConn *dbus.Conn) {
	apiHandler.Register("Config", &configAPI.Config{ConfigManager: configManager})
	apiHandler.Register("Firewall", &firewall.Firewall{ConfigManager: configManager})
	apiHandler.Register("Network", &network.Network{ConfigManager: configManager, DbusConn: dbusConn})
	apiHandler.Register("Object", &object.Object{ConfigManager: configManager})
	apiHandler.Register("Service", &service.Service{ConfigManager: configManager, DbusConn: dbusConn})
	apiHandler.Register("VPN", &vpn.VPN{ConfigManager: configManager, DbusConn: dbusConn})
	apiHandler.Register("System", &system.System{ConfigManager: configManager})
}

func RegisterApplyFunctions(configManager *config.ConfigManager) {
	configManager.RegisterApplyFunction(networkd.ApplyNetworkdConfiguration)
	configManager.RegisterApplyFunction(dhcp.ApplyDHCPServerConfiguration)
	configManager.RegisterApplyFunction(chrony.ApplyNTPConfiguration)
	configManager.RegisterApplyFunction(unbound.ApplyDNSServerConfiguration)
	configManager.RegisterApplyFunction(nftables.ApplyNFTablesConfiguration)
}
