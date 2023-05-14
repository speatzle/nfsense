package system

type System struct {
	Users map[string]User `json:"users" validate:"required,dive"`
}
