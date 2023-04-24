package util

import (
	"fmt"
	"os"
)

func OverwriteFile(path, content string) error {
	f, err := os.OpenFile(path, os.O_RDWR, 0644)
	if err != nil {
		return fmt.Errorf("opening File: %w", err)
	}

	err = f.Truncate(0)
	if err != nil {
		return fmt.Errorf("truncate File: %w", err)
	}

	_, err = f.Seek(0, 0)
	if err != nil {
		return fmt.Errorf("seek File: %w", err)
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
