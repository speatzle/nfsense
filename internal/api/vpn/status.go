package vpn

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"

	"golang.org/x/exp/slog"
)

type GetWireguardStatusResult struct {
	Status string
}

func (f *VPN) GetWireguardStatus(ctx context.Context, params struct{}) (GetWireguardStatusResult, error) {
	cmd := exec.Command("wg")
	var out bytes.Buffer
	cmd.Stdout = &out

	err := cmd.Run()
	if err != nil {
		return GetWireguardStatusResult{}, fmt.Errorf("restarting networkd: %w", err)
	}
	slog.Info("wg output", "out", out.String())
	return GetWireguardStatusResult{
		Status: out.String(),
	}, nil
}
