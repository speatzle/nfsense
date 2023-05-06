package vpn

import (
	"github.com/godbus/dbus/v5"
	"nfsense.net/nfsense/internal/config"
)

type VPN struct {
	ConfigManager *config.ConfigManager
	DbusConn      *dbus.Conn
}
