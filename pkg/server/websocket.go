package server

import (
	"context"
	"net/http"
	"time"

	"golang.org/x/exp/slog"
	"nhooyr.io/websocket"
	"nhooyr.io/websocket/wsjson"
)

func HandleWebsocketAPI(w http.ResponseWriter, r *http.Request) {
	c, err := websocket.Accept(w, r, nil)
	if err != nil {
		slog.Error("Accepting Websocket Connection", err)
		return
	}
	defer c.Close(websocket.StatusInternalError, "Unexpected Closing")

	ctx, cancel := context.WithTimeout(r.Context(), time.Second*10)
	defer cancel()

	var v interface{}
	err = wsjson.Read(ctx, c, &v)
	if err != nil {
		slog.Error("Reading Websocket Message", err)
		return
	}

	slog.Info("received websocket msg", "msg", v)

	c.Close(websocket.StatusNormalClosure, "")
}
