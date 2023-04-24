package unbound

import (
	"embed"
	"text/template"

	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/definitions/network"
)

//go:embed template
var templateFS embed.FS
var templates *template.Template

func init() {
	var err error
	templates, err = template.New("").Funcs(template.FuncMap{
		"getInterfaceNetworkAddressCIDR": getInterfaceNetworkAddressCIDR,
		"getInterfaceName":               getInterfaceName,
	}).ParseFS(templateFS, "template/*.tmpl")
	if err != nil {
		panic(err)
	}
}

func getInterfaceNetworkAddressCIDR(conf config.Config, name string) string {
	return conf.Network.Interfaces[name].Address.Masked().String()
}

func getInterfaceName(conf config.Config, name string) string {
	if conf.Network.Interfaces[name].Type == network.Hardware {
		return *conf.Network.Interfaces[name].HardwareDevice
	}
	return name
}
