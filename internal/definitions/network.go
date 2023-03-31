package definitions

type Network struct {
	Interfaces map[string]Interface `json:"interfaces" validate:"required,dive"`
}
