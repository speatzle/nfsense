package server

import (
	"bytes"
	"context"
	"fmt"
	"net/http"
	"runtime/debug"
	"time"

	"golang.org/x/exp/slog"
	"nhooyr.io/websocket"
)

func HandleWebsocketAPI(w http.ResponseWriter, r *http.Request) {
	_, s := GetSession(r)
	if s == nil {
		w.WriteHeader(http.StatusUnauthorized)
		return
	}

	ctx, cancel := context.WithCancel(context.WithValue(r.Context(), SessionKey, s))
	defer cancel()
	c, err := websocket.Accept(w, r, nil)
	if err != nil {
		slog.Error("Accepting Websocket Connection", err)
		return
	}
	defer c.Close(websocket.StatusInternalError, "Unexpected Closing")

	slog.Info("Accepted API Websocket Connection")

	for {
		_, m, err := c.Read(ctx)
		if websocket.CloseStatus(err) == websocket.StatusNormalClosure {
			slog.Info("API Websocket Closed Normally")
			cancel()
			return
		} else if err != nil {
			slog.Error("API Websocket Closed Unexpectedly", err)
			cancel()
		}

		go func() {
			defer func() {
				if r := recover(); r != nil {
					slog.Error("Recovered Panic Handling Websocket API Request", fmt.Errorf("%v", r), "stack", debug.Stack())
					return
				}
			}()
			ctx, cancel := context.WithTimeout(ctx, time.Second*10)
			defer cancel()

			err := apiHandler.HandleRequest(ctx, bytes.NewReader(m), w)
			if err != nil {
				slog.Error("Handling Websocket API Request", err)
			}
		}()
	}
}
