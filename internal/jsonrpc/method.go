package jsonrpc

import "reflect"

type method struct {
	subSystem   reflect.Value
	handlerFunc reflect.Value
	inType      reflect.Type
	outType     reflect.Type
}
