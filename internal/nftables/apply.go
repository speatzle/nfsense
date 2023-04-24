package nftables

import (
	"context"
	"fmt"

	systemctl "github.com/coreos/go-systemd/v22/dbus"
	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/util"
)

const nftablesFile = "/etc/nftables/nfsense.conf"

func ApplyNFTablesConfiguration(currentConfig config.Config, pendingConfig config.Config) error {
	nftablesConf, err := GenerateNfTablesConfig(pendingConfig)
	if err != nil {
		return fmt.Errorf("Generating nftables Configuration: %w", err)
	}

	err = util.OverwriteFile(nftablesFile, nftablesConf)
	if err != nil {
		return fmt.Errorf("Writing nftables Configuration: %w", err)
	}

	conn, err := systemctl.NewSystemConnectionContext(context.Background())
	if err != nil {
		return fmt.Errorf("Opening Dbus Connection: %w", err)
	}

	_, err = conn.ReloadOrRestartUnitContext(context.Background(), "nftables.service", "replace", nil)
	if err != nil {
		return fmt.Errorf("restarting unbound.service: %w", err)
	}
	return nil
}
