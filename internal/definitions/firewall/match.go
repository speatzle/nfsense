package firewall

type Match struct {
	TCPDestinationPort   uint64   `json:"tcp_destination_port,omitempty"`
	Services             []string `json:"services,omitempty"`
	SourceAddresses      []string `json:"source_addresses,omitempty"`
	DestinationAddresses []string `json:"destination_addresses,omitempty"`
}
