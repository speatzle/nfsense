package service

type DNSServer struct {
	Interface string `json:"interface"`
	Comment   string `json:"comment,omitempty"`
}
