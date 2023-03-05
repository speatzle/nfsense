package server

import (
	"context"
	"fmt"
	"net/http"
	"runtime/debug"
	"time"

	"golang.org/x/exp/slog"
)

func HandleAPI(w http.ResponseWriter, r *http.Request) {
	defer func() {
		if r := recover(); r != nil {
			slog.Error("Recovered Panic Handling HTTP API Request", fmt.Errorf("%v", r), "stack", debug.Stack())
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}
	}()
	ctx, cancel := context.WithTimeout(r.Context(), time.Second*10)
	defer cancel()

	err := apiHandler.HandleRequest(ctx, r.Body, w)
	if err != nil {
		slog.Error("Handling HTTP API Request", err)
	}
}
