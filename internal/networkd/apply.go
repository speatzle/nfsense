package networkd

import (
	"fmt"
	"os"

	"nfsense.net/nfsense/internal/definitions"
)

func ApplyNetworkdConfiguration(currentConfig definitions.Config, pendingConfig definitions.Config) error {
	files, err := GenerateNetworkdConfiguration(pendingConfig)
	if err != nil {
		return fmt.Errorf("Generating Networkd Configuration: %w", err)
	}

	for _, file := range files {
		f, err := os.Create("out/" + file.Name)
		if err != nil {
			return fmt.Errorf("creating File: %w", err)
		}

		_, err = f.WriteString(file.Content + "\n")
		if err != nil {
			return fmt.Errorf("writing File: %w", err)
		}

		err = f.Sync()
		if err != nil {
			return fmt.Errorf("syncing File: %w", err)
		}
	}
	return nil
}
