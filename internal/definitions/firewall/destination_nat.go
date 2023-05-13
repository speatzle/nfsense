package firewall

type DestinationNATRule struct {
	Rule
	Address *string `json:"address,omitempty"`
	Service *string `json:"service,omitempty"`
}
