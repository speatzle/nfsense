package config

import (
	"encoding/json"
	"fmt"

	"github.com/go-playground/validator/v10"
	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/definitions/firewall"
	"nfsense.net/nfsense/internal/definitions/network"
	"nfsense.net/nfsense/internal/definitions/object"
	"nfsense.net/nfsense/internal/definitions/service"
)

type Config struct {
	ConfigVersion uint64            `json:"config_version" validate:"required,eq=1"`
	Firewall      firewall.Firewall `json:"firewall" validate:"required,dive"`
	Object        object.Object     `json:"object" validate:"required,dive"`
	Network       network.Network   `json:"network" validate:"required,dive"`
	Service       service.Service   `json:"service" validate:"required,dive"`
}

// Clone TODO find a better way to deep copy
func (c *Config) Clone() *Config {
	data, err := json.Marshal(c)
	if err != nil {
		panic(fmt.Errorf("Marshal Error: %w", err))
	}
	var clone Config
	err = json.Unmarshal(data, &clone)
	if err != nil {
		panic(fmt.Errorf("Unmarshal Error: %w", err))
	}
	return &clone
}

func ValidateConfig(conf *Config) error {
	val := validator.New()
	val.RegisterValidation("test", nilIfOtherNil)
	return val.Struct(conf)
}

func nilIfOtherNil(fl validator.FieldLevel) bool {
	slog.Info("Start", "field", fl.FieldName(), "param", fl.Param())
	if !fl.Field().IsNil() {
		slog.Info("Field is not nil", "field", fl.FieldName())
		f := fl.Parent().FieldByName(fl.Param())
		if f.IsZero() {
			panic(fmt.Errorf("Param %v is not a Valid Field", fl.Param()))
		}
		if !f.IsNil() {
			slog.Info("Fail", "field", fl.FieldName(), "param", fl.Param())
			return false
		}
	}
	slog.Info("Success", "field", fl.FieldName(), "param", fl.Param())
	return true
}
