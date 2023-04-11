package network

import (
	"github.com/godbus/dbus/v5"
	"nfsense.net/nfsense/internal/config"
)

type Network struct {
	ConfigManager *config.ConfigManager
	DbusConn      *dbus.Conn
}
