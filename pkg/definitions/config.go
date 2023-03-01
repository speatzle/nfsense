package definitions

type Config struct {
	ConfigVersion uint64             `json:"config_version"`
	Netfilter     Netfilter          `json:"netfilter"`
	Addresses     map[string]Address `json:"addresses"`
	Services      map[string]Service `json:"services"`
}
