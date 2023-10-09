package validation

import (
	"embed"
	"fmt"
	"path/filepath"

	"github.com/santhosh-tekuri/jsonschema/v5"
)

//go:embed schemas/*.schema.json
var schemasFS embed.FS
var schema *jsonschema.Schema

func init() {
	all, err := schemasFS.ReadDir("schemas")
	if err != nil {
		panic(fmt.Errorf("Reading Schemas: %w", err))
	}

	c := jsonschema.NewCompiler()

	for _, f := range all {
		data, err := schemasFS.Open(filepath.Join("schemas", f.Name()))
		if err != nil {
			panic(fmt.Errorf("Reading Schema: %w", err))
		}
		err = c.AddResource("https://nfsense.net/"+f.Name(), data)
		if err != nil {
			panic(fmt.Errorf("Adding Schema: %w", err))
		}
	}

	s, err := c.Compile("https://nfsense.net/config.schema.json")
	if err != nil {
		panic(fmt.Errorf("Reading Schemas: %w", err))
	}

	schema = s
}
