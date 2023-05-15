package main

import (
	"context"
	"fmt"
	"net/netip"

	"github.com/godbus/dbus/v5"
	"github.com/pterm/pterm"
	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/api/network"
	"nfsense.net/nfsense/internal/api/system"
	"nfsense.net/nfsense/internal/config"
	netdefs "nfsense.net/nfsense/internal/definitions/network"
)

func setup(configManager *config.ConfigManager, dbusConn *dbus.Conn) {
	slog.Info("Entering Setup...")

	err := configManager.LoadDefaultConfig()
	if err != nil {
		slog.Error("Loading Default Config", "err", err)
		return
	}
	s := system.System{ConfigManager: configManager}
	n := network.Network{ConfigManager: configManager, DbusConn: dbusConn}

	slog.Info("Create Admin User")

	err = createUser(s)
	if err != nil {
		slog.Error("Error Creating User", "err", err)
		return
	}

	slog.Info("Configure LAN Interface")
	err = createInterface(n)
	if err != nil {
		slog.Error("Error Creating Interface", "err", err)
		return
	}

	slog.Info("Saving Changes...")
	err = configManager.SaveWithoutApplying()
	if err != nil {
		slog.Error("Saving", "err", err)
		return
	}

	slog.Info("Setup Done")
}

func createUser(s system.System) error {
	username, err := pterm.DefaultInteractiveTextInput.WithDefaultText("Username").Show()
	if err != nil {
		return fmt.Errorf("Reading Username Prompt: %w", err)
	}

password:

	password, err := pterm.DefaultInteractiveTextInput.WithDefaultText("Password").WithMask("*").Show()
	if err != nil {
		return fmt.Errorf("Reading Password Prompt: %w", err)
	}

	repeat, err := pterm.DefaultInteractiveTextInput.WithDefaultText("Repeat").WithMask("*").Show()
	if err != nil {
		return fmt.Errorf("Reading Repeat Prompt: %w", err)
	}

	if password != repeat {
		slog.Error("Password and Repeat Dont Match")
		goto password
	}

	_, err = s.CreateUser(context.Background(), system.CreateUserParameters{
		Name:     username,
		Password: password,
	})
	if err != nil {
		return fmt.Errorf("Doing API Call: %w", err)
	}
	return nil
}

func createInterface(n network.Network) error {
	links, err := n.GetLinks(context.Background(), struct{}{})
	if err != nil {
		return fmt.Errorf("Getting Links: %w", err)
	}

	options := []string{}

	for _, l := range links.Links {
		options = append(options, l.Name)
	}

	if len(options) == 0 {
		return fmt.Errorf("No Links Found")
	}

	link, err := pterm.DefaultInteractiveSelect.WithDefaultText("Select Hardware Interface").WithOptions(options).Show()
	if err != nil {
		return fmt.Errorf("Reading link Prompt: %w", err)
	}

	interfaceName, err := pterm.DefaultInteractiveTextInput.WithDefaultText("Interface Name").Show()
	if err != nil {
		return fmt.Errorf("Reading Interface Prompt: %w", err)
	}

	if interfaceName == "" {
		return fmt.Errorf("Interface Name Cannot be empty")
	}

	ipv4, err := pterm.DefaultInteractiveTextInput.WithDefaultText("Set IPv4 Address with CIDR").Show()
	if err != nil {
		return fmt.Errorf("Reading ipv4 Prompt: %w", err)
	}

	address, err := netip.ParsePrefix(ipv4)
	if err != nil {
		return fmt.Errorf("Parsing ipv4: %w", err)
	}

	_, err = n.CreateInterface(context.Background(), network.CreateInterfaceParameters{
		Name: interfaceName,
		Interface: netdefs.Interface{
			Type:           netdefs.Hardware,
			HardwareDevice: &link,
			AddressingMode: netdefs.Static,
			Address:        &address,
			Comment:        "Created by CLI Setup",
		},
	})
	if err != nil {
		return fmt.Errorf("Doing API Call: %w", err)
	}

	return nil
}
