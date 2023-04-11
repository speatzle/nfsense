package networkd

import (
	"embed"
	"text/template"
)

//go:embed template
var templateFS embed.FS
var templates *template.Template

func init() {

	var err error
	templates, err = template.New("").ParseFS(templateFS, "template/*.tmpl")
	if err != nil {
		panic(err)
	}
}
