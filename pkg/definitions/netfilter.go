package definitions

type Firewall struct {
	ForwardRules        []ForwardRule        `json:"forward_rules"`
	DestinationNATRules []DestinationNATRule `json:"destination_nat_rules"`
	SourceNATRules      []SourceNATRule      `json:"source_nat_rules"`
}
