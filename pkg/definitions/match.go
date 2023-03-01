package definitions

type Match struct {
	TCPDestinationPort uint64   `json:"tcp_destination_port,omitempty"`
	Services           []string `json:"services,omitempty"`
}
