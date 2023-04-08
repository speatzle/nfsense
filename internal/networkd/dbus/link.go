package dbus

import (
	"fmt"

	"github.com/godbus/dbus/v5"
	"golang.org/x/exp/slog"
)

type Link struct {
	Name             string `json:"name"`
	CarrierState     string `json:"carrier_state"`
	OperationalState string `json:"operational_state"`
}

func GetLinks(dbusConn dbus.Conn) ([]Link, error) {
	managerObj := dbusConn.Object("org.freedesktop.network1", dbus.ObjectPath("/org/freedesktop/network1"))

	var links [][]any
	err := managerObj.Call("org.freedesktop.network1.Manager.ListLinks", 0).Store(&links)
	if err != nil {
		return nil, fmt.Errorf("Calling ListLinks %w", err)
	}
	slog.Info("Dbus Result", "links", links)

	result := []Link{}

	for _, link := range links {
		name := link[1].(string)
		path := link[2].(string)
		linkObj := dbusConn.Object("org.freedesktop.network1", dbus.ObjectPath(path))
		carrierState, err := linkObj.GetProperty("CarrierState")
		if err != nil {
			return nil, fmt.Errorf("GetProperty CarrierState %w", err)
		}
		operationalState, err := linkObj.GetProperty("OperationalState")
		if err != nil {
			return nil, fmt.Errorf("GetProperty OperationalState %w", err)
		}
		result = append(result, Link{
			Name:             name,
			CarrierState:     carrierState.String(),
			OperationalState: operationalState.String(),
		})
	}

	return result, nil
}
