package validation

import (
	"embed"
	"fmt"
	"path/filepath"

	"github.com/santhosh-tekuri/jsonschema/v5"
	"golang.org/x/exp/slog"
)

//go:embed schema/*
var schemasFS embed.FS
var schema *jsonschema.Schema

func init() {

	c := jsonschema.NewCompiler()

	addFolderResources(c, "schema")

	s, err := c.Compile("https://nfsense.net/schema/config/config.schema.json")
	if err != nil {
		panic(fmt.Errorf("Reading Schemas: %w", err))
	}

	schema = s
}

func addFolderResources(c *jsonschema.Compiler, path string) {
	all, err := schemasFS.ReadDir(path)
	if err != nil {
		panic(fmt.Errorf("Reading Schemas: %w", err))
	}

	for _, f := range all {
		fullpath := filepath.Join(path, f.Name())
		slog.Debug("Checking Path", "fullpath", fullpath, "dir", f.IsDir())
		if f.IsDir() {
			addFolderResources(c, fullpath)
		} else {
			data, err := schemasFS.Open(fullpath)
			if err != nil {
				panic(fmt.Errorf("Reading Schema: %w", err))
			}
			slog.Debug("Adding Resource", "id", "https://nfsense.net/"+fullpath)
			err = c.AddResource("https://nfsense.net/"+fullpath, data)
			if err != nil {
				panic(fmt.Errorf("Adding Schema: %w", err))
			}
		}
	}
}
