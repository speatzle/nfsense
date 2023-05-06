package networkd

import (
	"embed"
	"text/template"

	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/definitions/object"
)

//go:embed template
var templateFS embed.FS
var templates *template.Template

func init() {

	var err error
	templates, err = template.New("").Funcs(template.FuncMap{
		"getAddressObjectsAsCidr": getAddressObjectsAsCidr,
	}).ParseFS(templateFS, "template/*.tmpl")
	if err != nil {
		panic(err)
	}
}

func getAddressObjectsAsCidr(conf config.Config, name string) string {
	addr := conf.Object.Addresses[name]
	switch addr.Type {
	case object.Host:
		return addr.Host.String() + "/32"
	case object.NetworkAddress:
		return addr.NetworkAddress.String()
	default:
		panic("unsupported Address Type")
	}
}
