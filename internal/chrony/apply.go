package chrony

import (
	"context"
	"fmt"

	systemctl "github.com/coreos/go-systemd/v22/dbus"
	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/util"
)

const chronyConfigFile = "/etc/chrony.conf"

func ApplyNTPConfiguration(currentConfig config.Config, pendingConfig config.Config) error {

	conf, err := GenerateChronyConfiguration(pendingConfig)
	if err != nil {
		return fmt.Errorf("Generating Chrony Configuration: %w", err)
	}

	err = util.OverwriteFile(chronyConfigFile, conf)
	if err != nil {
		return fmt.Errorf("Writing Chrony Configuration: %w", err)
	}

	conn, err := systemctl.NewSystemConnectionContext(context.Background())
	if err != nil {
		return fmt.Errorf("Opening Dbus Connection: %w", err)
	}

	_, err = conn.ReloadOrRestartUnitContext(context.Background(), "chronyd.service", "replace", nil)
	if err != nil {
		return fmt.Errorf("restarting chronyd.service: %w", err)
	}

	return nil
}
