package definitions

type Rule struct {
	Match   RuleMatch `json:"match"`
	Comment string    `json:"comment"`
	Counter bool      `json:"counter"`
}

type RuleMatch struct {
	TCPDestinationPort uint64 `json:"tcp_destination_port"`
}

type ForwardRule struct {
	Rule
}

type DestinationNATRule struct {
	Rule
}

type SourceNATRule struct {
	Rule
}
