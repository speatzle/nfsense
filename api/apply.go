package main

import (
	"fmt"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/pkg/definitions"
	"nfsense.net/nfsense/pkg/nftables"
)

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
