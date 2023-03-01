package definitions

type Config struct {
	ConfigVersion int64     `json:"config_version"`
	Netfilter     Netfilter `json:"netfilter"`
}
