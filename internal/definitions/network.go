package definitions

type Network struct {
	Interfaces []Interface `json:"interfaces" validate:"required,dive"`
}
