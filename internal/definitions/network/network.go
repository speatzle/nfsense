package network

type Network struct {
	Interfaces   map[string]Interface `json:"interfaces"`
	StaticRoutes []StaticRoute        `json:"static_routes"`
}
