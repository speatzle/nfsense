package service

type NTPServer struct {
	Interface string `json:"interface"`
	Comment   string `json:"comment,omitempty"`
}
