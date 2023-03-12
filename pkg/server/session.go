package server

import (
	"encoding/json"
	"io"
	"net/http"
	"time"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/pkg/session"
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
		slog.Error("Unmarshal", err)
		return
	}
	if req.Username == "admin" && req.Password == "12345" {
		slog.Info("User Login Successfull")
		session.GenerateSession(w, req.Username)
		w.WriteHeader(http.StatusOK)
		return
	}
	w.WriteHeader(http.StatusUnauthorized)
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
