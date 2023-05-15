package server

import (
	"encoding/json"
	"io"
	"net/http"
	"time"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/auth"
	"nfsense.net/nfsense/internal/session"
)

type LoginRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

func HandleLogin(w http.ResponseWriter, r *http.Request) {
	buf, err := io.ReadAll(r.Body)
	if err != nil {
		slog.Error("Reading Body", err)
		return
	}
	var req LoginRequest
	err = json.Unmarshal(buf, &req)
	if err != nil {
		slog.Error("Unmarshal", "err", err)
		return
	}
	err = auth.AuthenticateUser(configManager.GetCurrentConfig(), req.Username, req.Password)
	if err != nil {
		slog.Error("User Login failed", "err", err, "username", req.Username)
		w.WriteHeader(http.StatusUnauthorized)
		return
	}

	slog.Info("User Login Successful", "username", req.Username)
	session.GenerateSession(w, req.Username)
	w.WriteHeader(http.StatusOK)
}

func HandleLogout(w http.ResponseWriter, r *http.Request) {
	http.SetCookie(w, session.GetCookie("", time.Now()))
	w.WriteHeader(http.StatusOK)
}

func HandleSession(w http.ResponseWriter, r *http.Request) {
	id, s := session.GetSession(r)
	if s == nil {
		w.WriteHeader(http.StatusUnauthorized)
		return
	}
	session.ExtendSession(s)
	http.SetCookie(w, session.GetCookie(id, s.Expires))
	w.WriteHeader(http.StatusOK)
	resp := session.SessionResponse{
		CommitHash: session.CommitHash,
	}
	res, err := json.Marshal(resp)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.Write(res)
}
