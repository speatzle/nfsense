package vpn

type Wireguard struct {
	Interfaces map[string]WireguardInterface `json:"interfaces"`
	Peers      map[string]WireguardPeer      `json:"peers"`
}
