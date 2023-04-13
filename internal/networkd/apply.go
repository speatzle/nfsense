package networkd

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/definitions/config"
)

const basepath = "/etc/systemd/network"

func ApplyNetworkdConfiguration(currentConfig config.Config, pendingConfig config.Config) error {
	files, err := GenerateNetworkdConfiguration(pendingConfig)
	if err != nil {
		return fmt.Errorf("Generating Networkd Configuration: %w", err)
	}

	err = RemoveContents(basepath)
	if err != nil {
		return fmt.Errorf("Removing old Config Files: %w", err)
	}

	for _, file := range files {
		f, err := os.Create(basepath + "/" + file.Name)
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

	// TODO Use dbus instead
	cmd := exec.Command("systemctl", "restart", "systemd-networkd")

	var out bytes.Buffer
	cmd.Stdout = &out

	err = cmd.Run()
	if err != nil {
		return fmt.Errorf("restarting networkd: %w", err)
	}
	slog.Info("networkd output", "out", out.String())

	return nil
}

func RemoveContents(dir string) error {
	d, err := os.Open(dir)
	if err != nil {
		return err
	}
	defer d.Close()
	names, err := d.Readdirnames(-1)
	if err != nil {
		return err
	}
	for _, name := range names {
		err = os.RemoveAll(filepath.Join(dir, name))
		if err != nil {
			return err
		}
	}
	return nil
}
