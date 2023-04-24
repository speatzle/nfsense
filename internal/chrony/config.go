package chrony

import (
	"bytes"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/config"
)

func GenerateChronyConfiguration(conf config.Config) (string, error) {
	buf := new(bytes.Buffer)
	err := templates.ExecuteTemplate(buf, "config.tmpl", conf)
	if err != nil {
		return "", fmt.Errorf("executing server.tmpl template: %w", err)
	}
	return buf.String(), nil
}
