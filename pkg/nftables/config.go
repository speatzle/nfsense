package nftables

import (
	"bytes"
	"fmt"
	"os"

	"github.con/speatzle/nfsense/pkg/definitions"
)

func GenerateNfTablesFile(conf definitions.Config) (string, error) {
	buf := new(bytes.Buffer)
	err := templates.ExecuteTemplate(buf, "nftables.tmpl", conf)
	if err != nil {
		return "", fmt.Errorf("executing template: %w", err)
	}
	return buf.String(), nil
}

func ApplyNfTablesFile(content string) error {
	f, err := os.Create("nftables.conf")
	if err != nil {
		return fmt.Errorf("creating File: %w", err)
	}

	_, err = f.WriteString(content + "\n")
	if err != nil {
		return fmt.Errorf("writing File: %w", err)
	}

	err = f.Sync()
	if err != nil {
		return fmt.Errorf("syncing File: %w", err)
	}
	return nil
}
