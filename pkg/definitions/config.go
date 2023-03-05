package definitions

type Config struct {
	ConfigVersion uint64   `json:"config_version"`
	Firewall      Firewall `json:"firewall"`
}
