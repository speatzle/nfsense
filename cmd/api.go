package main

import (
	"nfsense.net/nfsense/api/firewall"
	"nfsense.net/nfsense/pkg/definitions"
	"nfsense.net/nfsense/pkg/jsonrpc"
)

func RegisterAPIMethods(apiHandler *jsonrpc.Handler, conf *definitions.Config) {
	apiHandler.Register("Firewall", &firewall.Firewall{Conf: conf})
}
