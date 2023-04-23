package networkd

import (
	"bytes"
	"fmt"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/definitions/network"
)

type NetworkdConfigFile struct {
	Name    string
	Content string
}

type InterfaceWithName struct {
	Name string
	network.Interface
	Vlans        []string
	StaticRoutes []network.StaticRoute
}

type BondMembership struct {
	Name     string
	BondName string
}

type BridgeMembership struct {
	Name       string
	BridgeName string
}

func GenerateNetworkdConfiguration(conf config.Config) ([]NetworkdConfigFile, error) {
	files := []NetworkdConfigFile{}

	// Step 1 Generate vlan netdev files
	for name, inter := range conf.Network.Interfaces {
		if inter.Type == network.Vlan {
			buf := new(bytes.Buffer)
			err := templates.ExecuteTemplate(buf, "create-vlan.netdev.tmpl", InterfaceWithName{
				Name:      name,
				Interface: inter,
			})
			if err != nil {
				return nil, fmt.Errorf("executing create-vlan.netdev.tmpl template: %w", err)
			}
			files = append(files, NetworkdConfigFile{
				Name:    fmt.Sprintf("10-create-vlan-%v.netdev", name),
				Content: buf.String(),
			})
		}
	}

	// Step 2 Generate bond netdev files
	for name, inter := range conf.Network.Interfaces {
		if inter.Type == network.Bond {
			buf := new(bytes.Buffer)
			err := templates.ExecuteTemplate(buf, "create-bond.netdev.tmpl", InterfaceWithName{
				Name:      name,
				Interface: inter,
			})
			if err != nil {
				return nil, fmt.Errorf("executing create-bond.netdev.tmpl template: %w", err)
			}
			files = append(files, NetworkdConfigFile{
				Name:    fmt.Sprintf("20-create-bond-%v.netdev", name),
				Content: buf.String(),
			})
		}
	}

	// Step 3 Generate bridge netdev files
	for name, inter := range conf.Network.Interfaces {
		if inter.Type == network.Bridge {
			buf := new(bytes.Buffer)
			err := templates.ExecuteTemplate(buf, "create-bridge.netdev.tmpl", InterfaceWithName{
				Name:      name,
				Interface: inter,
			})
			if err != nil {
				return nil, fmt.Errorf("executing create-bridge.netdev.tmpl template: %w", err)
			}
			files = append(files, NetworkdConfigFile{
				Name:    fmt.Sprintf("30-create-bridge-%v.netdev", name),
				Content: buf.String(),
			})
		}
	}

	// Step 4 Generate Bond Members
	for name, inter := range conf.Network.Interfaces {
		if inter.Type == network.Bond && inter.BondMembers != nil {
			for _, member := range *inter.BondMembers {
				buf := new(bytes.Buffer)
				err := templates.ExecuteTemplate(buf, "bond-membership.network.tmpl", BondMembership{
					Name:     member,
					BondName: name,
				})
				if err != nil {
					return nil, fmt.Errorf("executing bond-membership.network.tmpl template: %w", err)
				}
				files = append(files, NetworkdConfigFile{
					Name:    fmt.Sprintf("40-bond-membership-%v.network", name),
					Content: buf.String(),
				})
			}
		}
	}

	// Step 5 Generate Bridge Members
	for name, inter := range conf.Network.Interfaces {
		if inter.Type == network.Bridge && inter.BridgeMembers != nil {
			for _, member := range *inter.BridgeMembers {
				buf := new(bytes.Buffer)
				err := templates.ExecuteTemplate(buf, "bridge-membership.network.tmpl", BridgeMembership{
					Name:       member,
					BridgeName: name,
				})
				if err != nil {
					return nil, fmt.Errorf("executing bridge-membership.network.tmpl template: %w", err)
				}
				files = append(files, NetworkdConfigFile{
					Name:    fmt.Sprintf("50-bridge-membership-%v.network", name),
					Content: buf.String(),
				})
			}
		}
	}

	// Step 6 Generate addressing network files
	for name, inter := range conf.Network.Interfaces {
		// Vlans
		vlans := []string{}
		if inter.Type != network.Vlan {
			for vlanName, vlanInter := range conf.Network.Interfaces {
				if vlanInter.Type == network.Vlan {
					if *vlanInter.VlanParent == name {
						vlans = append(vlans, vlanName)
					}
				}
			}
			slog.Info("Vlans on interface", "interface", name, "count", len(vlans))
		}

		// Static Routes
		staticRoutes := []network.StaticRoute{}
		for _, route := range conf.Network.StaticRoutes {
			if route.Interface == name {
				staticRoutes = append(staticRoutes, route)
			}
		}

		buf := new(bytes.Buffer)
		err := templates.ExecuteTemplate(buf, "config-addressing.network.tmpl", InterfaceWithName{
			Name:         name,
			Interface:    inter,
			Vlans:        vlans,
			StaticRoutes: staticRoutes,
		})
		if err != nil {
			return nil, fmt.Errorf("executing config-addressing.network.tmpl template: %w", err)
		}
		files = append(files, NetworkdConfigFile{
			Name:    fmt.Sprintf("60-config-addressing-%v.network", name),
			Content: buf.String(),
		})
	}

	return files, nil
}
