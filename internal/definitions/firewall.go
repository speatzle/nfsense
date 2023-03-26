package definitions

type Firewall struct {
	ForwardRules        []ForwardRule        `json:"forward_rules" validate:"required,dive"`
	DestinationNATRules []DestinationNATRule `json:"destination_nat_rules" validate:"required,dive"`
	SourceNATRules      []SourceNATRule      `json:"source_nat_rules" validate:"required,dive"`
	Addresses           map[string]Address   `json:"addresses" validate:"required,dive"`
	Services            map[string]Service   `json:"services" validate:"required,dive"`
}
