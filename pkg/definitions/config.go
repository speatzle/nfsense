package definitions

type Config struct {
	ConfigVersion int64     `json:"config_version"`
	Netfilter     Netfilter `json:"netfilter"`
	Netfilter     Netfilter          `json:"netfilter"`
	Addresses     map[string]Address `json:"addresses"`
	Services      map[string]Service `json:"services"`
}
