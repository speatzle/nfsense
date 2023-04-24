package dhcp

import (
	"context"
	"fmt"

	systemctl "github.com/coreos/go-systemd/v22/dbus"
	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/util"
)

const dhcpv4File = "/etc/dhcp/dhcpd.conf"
const dhcpv6File = "/etc/dhcp/dhcpd6.conf"

func ApplyDHCPServerConfiguration(currentConfig config.Config, pendingConfig config.Config) error {

	v4Conf, err := GenerateDHCPServerV4Configuration(pendingConfig)
	if err != nil {
		return fmt.Errorf("Generating DHCPServerV4 Configuration: %w", err)
	}

	v6Conf, err := GenerateDHCPServerV6Configuration(pendingConfig)
	if err != nil {
		return fmt.Errorf("Generating DHCPServerV6 Configuration: %w", err)
	}

	err = util.OverwriteFile(dhcpv4File, v4Conf)
	if err != nil {
		return fmt.Errorf("Writing v4 Configuration: %w", err)
	}

	err = util.OverwriteFile(dhcpv6File, v6Conf)
	if err != nil {
		return fmt.Errorf("Writing v6 Configuration: %w", err)
	}

	conn, err := systemctl.NewSystemConnectionContext(context.Background())
	if err != nil {
		return fmt.Errorf("Opening Dbus Connection: %w", err)
	}

	if len(pendingConfig.Service.DHCPv4Servers) == 0 && len(pendingConfig.Service.DHCPv6Servers) == 0 {
		// if there are no servers stop the service instead
		_, err := conn.StopUnitContext(context.Background(), "dhcpd.service", "replace", nil)
		if err != nil {
			return fmt.Errorf("stopping dhcpd.service: %w", err)
		}

		_, err = conn.DisableUnitFilesContext(context.Background(), []string{"dhcpd.service"}, false)
		if err != nil {
			return fmt.Errorf("disableing dhcpd.service: %w", err)
		}
	} else {
		_, err := conn.ReloadOrRestartUnitContext(context.Background(), "dhcpd.service", "replace", nil)
		if err != nil {
			return fmt.Errorf("restarting dhcpd.service: %w", err)
		}

		_, _, err = conn.EnableUnitFilesContext(context.Background(), []string{"dhcpd.service"}, false, true)
		if err != nil {
			return fmt.Errorf("enableing dhcpd.service: %w", err)
		}
	}
	return nil
}
