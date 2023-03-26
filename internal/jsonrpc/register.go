package jsonrpc

import (
	"context"
	"fmt"
	"reflect"
)

func (h *Handler) Register(subSystemName string, s any) {
	subSystem := reflect.ValueOf(s)

	for i := 0; i < subSystem.NumMethod(); i++ {
		m := subSystem.Type().Method(i)

		funcType := m.Func.Type()

		if funcType.NumIn() != 3 {
			panic(fmt.Errorf("2 parameters are required %v", funcType.NumIn()))
		}
		if funcType.In(1) != reflect.TypeOf(new(context.Context)).Elem() {
			panic(fmt.Errorf("the first argument needs to be a context.Context instead of %v ", funcType.In(1)))
		}
		if funcType.In(2).Kind() != reflect.Struct {
			panic("the second argument needs to be a struct")
		}

		if funcType.NumOut() != 2 {
			panic("2 return types are required")
		}
		if reflect.TypeOf(new(error)).Implements(funcType.Out(1)) {
			panic("the second return type needs to be a error")
		}

		name := m.Name
		if subSystemName != "" {
			name = subSystemName + "." + name
		}

		h.methods[name] = method{
			handlerFunc: m.Func,
			subSystem:   subSystem,
			inType:      funcType.In(2),
			outType:     funcType.Out(0),
		}
	}
}
