package definitions

import "fmt"

type Match struct {
	TCPDestinationPort uint64   `json:"tcp_destination_port,omitempty"`
	Service            []string `json:"service,omitempty"`
}

func (m Match) Nftables() string {
	return fmt.Sprintf("tcp dport %d", m.TCPDestinationPort)
}
