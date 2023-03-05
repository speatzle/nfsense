package definitions

type Firewall struct {
	ForwardRules        []ForwardRule        `json:"forward_rules"`
	DestinationNATRules []DestinationNATRule `json:"destination_nat_rules"`
	SourceNATRules      []SourceNATRule      `json:"source_nat_rules"`
	Addresses           map[string]Address   `json:"addresses"`
	Services            map[string]Service   `json:"services"`
}
