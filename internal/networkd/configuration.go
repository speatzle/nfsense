package networkd

import (
	"bytes"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
)

type NetworkdConfigFile struct {
	Name    string
	Content string
}

type InterfaceWithName struct {
	Name string
	definitions.Interface
}

type BondMembership struct {
	Name     string
	BondName string
}

type BridgeMembership struct {
	Name       string
	BridgeName string
}

type VlanAssignments struct {
	Name  string
	Vlans []string
}

func GenerateNetworkdConfiguration(conf definitions.Config) ([]NetworkdConfigFile, error) {
	files := []NetworkdConfigFile{}

	// Step 1 Generate vlan netdev files
	for name, inter := range conf.Network.Interfaces {
		if inter.Type == definitions.Vlan {
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
		if inter.Type == definitions.Bond {
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
		if inter.Type == definitions.Bridge {
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
		if inter.Type == definitions.Bond && inter.BondMembers != nil {
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
		if inter.Type == definitions.Bridge && inter.BridgeMembers != nil {
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

	// Step 6 Generate Vlan Assignments
	for name, inter := range conf.Network.Interfaces {
		if inter.Type != definitions.Vlan {
			vlans := []string{}
			for vlanName, vlanInter := range conf.Network.Interfaces {
				if inter.Type == definitions.Vlan && *vlanInter.VlanParent == name {
					vlans = append(vlans, vlanName)
				}
			}

			if len(vlans) != 0 {
				buf := new(bytes.Buffer)
				err := templates.ExecuteTemplate(buf, "vlan-assignments.network.tmpl", VlanAssignments{
					Name:  name,
					Vlans: vlans,
				})
				if err != nil {
					return nil, fmt.Errorf("executing vlan-assignments.network.tmpl template: %w", err)
				}
				files = append(files, NetworkdConfigFile{
					Name:    fmt.Sprintf("60-bridge-membership-%v.network", name),
					Content: buf.String(),
				})
			}
		}
	}

	// Step 7 Generate addressing network files
	for name, inter := range conf.Network.Interfaces {
		buf := new(bytes.Buffer)
		err := templates.ExecuteTemplate(buf, "config-addressing.network.tmpl", InterfaceWithName{
			Name:      name,
			Interface: inter,
		})
		if err != nil {
			return nil, fmt.Errorf("executing config-addressing.network.tmpl template: %w", err)
		}
		files = append(files, NetworkdConfigFile{
			Name:    fmt.Sprintf("70-config-addressing-%v.network", name),
			Content: buf.String(),
		})
	}

	return files, nil
}
