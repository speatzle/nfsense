package vpn

type Wireguard struct {
	Interfaces map[string]WireguardInterface `json:"interfaces" validate:"required,dive"`
	Peers      map[string]WireguardPeer      `json:"peers" validate:"required,dive"`
}
