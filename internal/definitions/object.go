package definitions

type Object struct {
	Addresses map[string]Address `json:"addresses" validate:"required,dive"`
	Services  map[string]Service `json:"services" validate:"required,dive"`
}
