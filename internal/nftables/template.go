package nftables

import (
	"embed"
	"text/template"
)

//go:embed template
var templateFS embed.FS
var templates *template.Template

func init() {

	funcMap := template.FuncMap{
		// The name "title" is what the function will be called in the template text.
		"matcher": GenerateMatcher,
	}

	var err error
	templates, err = template.New("").Funcs(funcMap).ParseFS(templateFS, "template/*.tmpl")
	if err != nil {
		panic(err)
	}
}
