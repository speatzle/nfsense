package nftables

import (
	"bytes"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/config"
)

func GenerateNfTablesConfig(conf config.Config) (string, error) {
	buf := new(bytes.Buffer)
	err := templates.ExecuteTemplate(buf, "nftables.tmpl", conf)
	if err != nil {
		return "", fmt.Errorf("executing template: %w", err)
	}
	return buf.String(), nil
}
