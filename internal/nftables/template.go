package nftables

import (
	"embed"
	"text/template"

	"nfsense.net/nfsense/internal/util"
)

//go:embed template
var templateFS embed.FS
var templates *template.Template

func init() {

	funcMap := template.FuncMap{
		"addressMatcher":       GenerateAddressMatcher,
		"serviceMatcher":       GenerateServiceMatcher,
		"destinationNatAction": GenerateDestinationNatAction,
		"sourceNatAction":      GenerateSourceNatAction,
		"getBaseServices":      util.ResolveBaseServices,
	}

	var err error
	templates, err = template.New("").Funcs(funcMap).ParseFS(templateFS, "template/*.tmpl")
	if err != nil {
		panic(err)
	}
}
