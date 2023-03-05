package definitions

type Config struct {
	ConfigVersion uint64             `json:"config_version"`
	Firewall      Firewall           `json:"firewall"`
	Addresses     map[string]Address `json:"addresses"`
	Services      map[string]Service `json:"services"`
}
