package unbound

import (
	"context"
	"fmt"

	systemctl "github.com/coreos/go-systemd/v22/dbus"
	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/util"
)

const unboundServerFile = "/etc/unbound/unbound.conf"

func ApplyDNSServerConfiguration(currentConfig config.Config, pendingConfig config.Config) error {

	serverConf, err := GenerateUnboundServerConfiguration(pendingConfig)
	if err != nil {
		return fmt.Errorf("Generating Unbound Server Configuration: %w", err)
	}

	err = util.OverwriteFile(unboundServerFile, serverConf)
	if err != nil {
		return fmt.Errorf("Writing server Configuration: %w", err)
	}

	conn, err := systemctl.NewSystemConnectionContext(context.Background())
	if err != nil {
		return fmt.Errorf("Opening Dbus Connection: %w", err)
	}

	if len(pendingConfig.Service.DNSServers) == 0 {
		// if there are no servers stop the service instead
		_, err := conn.StopUnitContext(context.Background(), "unbound.service", "replace", nil)
		if err != nil {
			return fmt.Errorf("stopping unbound.service: %w", err)
		}

		_, err = conn.DisableUnitFilesContext(context.Background(), []string{"unbound.service"}, false)
		if err != nil {
			return fmt.Errorf("disableing unbound.service: %w", err)
		}
	} else {
		_, err := conn.ReloadOrRestartUnitContext(context.Background(), "unbound.service", "replace", nil)
		if err != nil {
			return fmt.Errorf("restarting unbound.service: %w", err)
		}

		_, _, err = conn.EnableUnitFilesContext(context.Background(), []string{"unbound.service"}, false, true)
		if err != nil {
			return fmt.Errorf("enableing unbound.service: %w", err)
		}
	}
	return nil
}
