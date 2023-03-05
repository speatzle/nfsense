package server

import (
	"context"
	"fmt"
	"net/http"
	"runtime/debug"

	"golang.org/x/exp/slog"
)

func HandleAPI(w http.ResponseWriter, r *http.Request) {
	defer func() {
		if r := recover(); r != nil {
			slog.Error("Recovered Panic Handling HTTP API Request", fmt.Errorf("%v", r), "stack", debug.Stack())
		}
	}()
	err := apiHandler.HandleRequest(context.TODO(), r.Body, w)
	if err != nil {
		slog.Error("Handling HTTP API Request", err)
	}
}
