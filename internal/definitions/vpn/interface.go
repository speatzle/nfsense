package vpn

type WireguardInterface struct {
	PublicKey  string   `json:"public_key"`
	PrivateKey string   `json:"private_key"`
	ListenPort uint64   `json:"listen_port"`
	Peers      []string `json:"peers"`
	Comment    string   `json:"comment,omitempty"`
}
