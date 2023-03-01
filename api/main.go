package main

import (
	"github.con/speatzle/nfsense/pkg/nftables"
	"golang.org/x/exp/slog"
)

func main() {
	slog.Info("Starting...")
	conf, err := LoadConfiguration("config.json")
	if err != nil {
		slog.Error("Loading Config", err)
		return
	}
	slog.Info("Config Loaded", "config", conf)

	fileContent, err := nftables.GenerateNfTablesFile(*conf)
	if err != nil {
		slog.Error("Generating nftables file", err)
		return
	}

	err = nftables.ApplyNfTablesFile(fileContent)
	if err != nil {
		slog.Error("Applying nftables", err)
		return
	}
	slog.Info("Wrote nftables File!")
}
