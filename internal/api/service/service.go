package service

import (
	"github.com/godbus/dbus/v5"
	"nfsense.net/nfsense/internal/config"
)

type Service struct {
	ConfigManager *config.ConfigManager
	DbusConn      *dbus.Conn
}
