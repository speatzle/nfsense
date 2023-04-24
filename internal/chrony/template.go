package chrony

import (
	"embed"
	"text/template"

	"nfsense.net/nfsense/internal/definitions/config"
)

//go:embed template
var templateFS embed.FS
var templates *template.Template

func init() {
	var err error
	templates, err = template.New("").Funcs(template.FuncMap{
		"getInterfaceNetworkAddressCIDR": getInterfaceNetworkAddressCIDR,
	}).ParseFS(templateFS, "template/*.tmpl")
	if err != nil {
		panic(err)
	}
}

func getInterfaceNetworkAddressCIDR(conf config.Config, name string) string {
	return conf.Network.Interfaces[name].Address.Masked().String()
}
