package definitions

import (
	"fmt"

	"github.com/go-playground/validator/v10"
	"golang.org/x/exp/slog"
)

type Config struct {
	ConfigVersion uint64   `json:"config_version" validate:"required,eq=1"`
	Firewall      Firewall `json:"firewall" validate:"required,dive"`
	Object        Object   `json:"object" validate:"required,dive"`
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
