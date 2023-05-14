package system

type User struct {
	Comment string `json:"comment"`
	Hash    string `json:"hash"`
	Salt    string `json:"salt"`
}
