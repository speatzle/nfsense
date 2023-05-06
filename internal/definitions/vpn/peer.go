package vpn

type WireguardPeer struct {
	PublicKey           string   `json:"public_key"`
	PresharedKey        *string  `json:"preshared_key,omitempty"`
	AllowedIPs          []string `json:"allowed_ips"`
	Endpoint            *string  `json:"endpoint,omitempty"`
	PersistentKeepalive *uint64  `json:"persistent_keepalive,omitempty"`
	Comment             string   `json:"comment,omitempty"`
}
