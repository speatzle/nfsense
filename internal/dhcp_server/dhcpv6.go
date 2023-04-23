package dhcp

import (
	"bytes"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/config"
)

func GenerateDHCPServerV6Configuration(conf config.Config) (string, error) {
	buf := new(bytes.Buffer)
	err := templates.ExecuteTemplate(buf, "v6_config.tmpl", conf)
	if err != nil {
		return "", fmt.Errorf("executing config.tmpl template: %w", err)
	}
	return buf.String(), nil
}
