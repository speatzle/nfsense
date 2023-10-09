package object

type Object struct {
	Addresses map[string]Address `json:"addresses"`
	Services  map[string]Service `json:"services"`
}
