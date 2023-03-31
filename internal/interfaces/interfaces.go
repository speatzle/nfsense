package interfaces

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"

	"nfsense.net/nfsense/internal/definitions"
)

func GenerateInterfacesFile(conf definitions.Config) (string, error) {
	buf := new(bytes.Buffer)
	err := templates.ExecuteTemplate(buf, "interfaces.tmpl", conf)
	if err != nil {
		return "", fmt.Errorf("executing template: %w", err)
	}
	return buf.String(), nil
}

func ApplyInterfacesFile(content string) (string, error) {
	f, err := os.Create("interfaces.conf")
	if err != nil {
		return "", fmt.Errorf("creating File: %w", err)
	}

	_, err = f.WriteString(content + "\n")
	if err != nil {
		return "", fmt.Errorf("writing File: %w", err)
	}

	err = f.Sync()
	if err != nil {
		return "", fmt.Errorf("syncing File: %w", err)
	}

	cmd := exec.Command("ifreload", "-a")

	var out bytes.Buffer
	cmd.Stdout = &out

	err = cmd.Run()
	if err != nil {
		return "", fmt.Errorf("reloading Interfaces: %w", err)
	}
	return out.String(), nil
}
