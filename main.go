package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"
	"time"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/api/firewall"
	"nfsense.net/nfsense/internal/api/object"
	"nfsense.net/nfsense/internal/definitions"
	"nfsense.net/nfsense/internal/jsonrpc"
	"nfsense.net/nfsense/internal/nftables"
	"nfsense.net/nfsense/internal/server"
)

func main() {
	applyPtr := flag.Bool("apply", false, "apply config and stop")
	flag.Parse()

	slog.Info("Starting...")

	conf, err := LoadConfiguration("config.json")
	if err != nil {
		slog.Error("Loading Config", err)
		os.Exit(1)
	}

	slog.Info("Config Loaded", "config", conf)

	err = definitions.ValidateConfig(conf)
	if err != nil {
		slog.Error("Validating Config", err)
		os.Exit(1)
	}

	slog.Info("Validating Config...")

	if *applyPtr {
		slog.Info("Applying Config...")
		err := apply(conf)
		if err != nil {
			slog.Error("Applying Config", err)
			os.Exit(1)
		}
		slog.Info("Config Applied, Exiting...")
		return
	}

	slog.Info("Setup API...")
	apiHandler := jsonrpc.NewHandler(100 << 20)
	RegisterAPIMethods(apiHandler, conf)

	slog.Info("Starting Webserver...")
	server.StartWebserver(conf, apiHandler)

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

func LoadConfiguration(file string) (*definitions.Config, error) {
	var config definitions.Config
	configFile, err := os.Open(file)
	if err != nil {
		return nil, fmt.Errorf("opening Config File %w", err)
	}
	defer configFile.Close()

	jsonParser := json.NewDecoder(configFile)
	jsonParser.DisallowUnknownFields()
	err = jsonParser.Decode(&config)
	if err != nil {
		return nil, fmt.Errorf("decoding Config File %w", err)
	}
	return &config, nil
}

func RegisterAPIMethods(apiHandler *jsonrpc.Handler, conf *definitions.Config) {
	apiHandler.Register("Firewall", &firewall.Firewall{Conf: conf})
	apiHandler.Register("Object", &object.Object{Conf: conf})
}

func apply(conf *definitions.Config) error {
	fileContent, err := nftables.GenerateNfTablesFile(*conf)
	if err != nil {
		return fmt.Errorf("Generating nftables file %w", err)
	}

	err = nftables.ApplyNfTablesFile(fileContent)
	if err != nil {
		return fmt.Errorf("Applying nftables %w", err)
	}
	slog.Info("Wrote nftables File!")
	return nil
}
