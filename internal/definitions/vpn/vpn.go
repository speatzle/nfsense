package vpn

type VPN struct {
	Wireguard Wireguard `json:"wireguard" validate:"required,dive"`
}
