package server

import (
	"encoding/json"
	"io"
	"net/http"
	"runtime/debug"
	"sync"
	"time"

	"github.com/google/uuid"
	"golang.org/x/exp/slog"
)

type SessionKeyType string

const SessionKey SessionKeyType = "session"
const SessionCookieName string = "session"

type Session struct {
	Username string
	Expires  time.Time
	// TODO Add []websocket.Conn pointer to close all active websockets, alternativly do this via context cancelation
}

type LoginRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type SessionResponse struct {
	CommitHash string `json:"commit_hash"`
}

var sessionsSync sync.Mutex
var sessions map[string]*Session = map[string]*Session{}

var CommitHash = func() string {
	if info, ok := debug.ReadBuildInfo(); ok {
		for _, setting := range info.Settings {
			if setting.Key == "vcs.revision" {
				return setting.Value
			}
		}
	}
	return "asd"
}()

func GetSession(r *http.Request) (string, *Session) {
	c, err := r.Cookie("session")
	if err != nil {
		return "", nil
	}
	s, ok := sessions[c.Value]
	if ok {
		return c.Value, s
	}
	return "", nil
}

func GenerateSession(w http.ResponseWriter, username string) {
	id := uuid.New().String()
	expires := time.Now().Add(time.Minute * 5)
	sessionsSync.Lock()
	defer sessionsSync.Unlock()
	sessions[id] = &Session{
		Username: username,
		Expires:  expires,
	}
	http.SetCookie(w, &http.Cookie{Name: SessionCookieName, HttpOnly: true, SameSite: http.SameSiteStrictMode, Value: id, Expires: expires})
}

func CleanupSessions(stop chan struct{}) {
	tick := time.NewTicker(time.Minute)
	for {
		select {
		case <-tick.C:
			ids := []string{}
			sessionsSync.Lock()
			for id, s := range sessions {
				if time.Now().After(s.Expires) {
					ids = append(ids, id)
				}
			}
			for _, id := range ids {
				delete(sessions, id)
			}
			sessionsSync.Unlock()
		case <-stop:
			return
		}
	}
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
		GenerateSession(w, req.Username)
		w.WriteHeader(http.StatusOK)
		return
	}
	w.WriteHeader(http.StatusUnauthorized)
}

func HandleLogout(w http.ResponseWriter, r *http.Request) {
	http.SetCookie(w, &http.Cookie{Name: SessionCookieName, HttpOnly: true, SameSite: http.SameSiteStrictMode, Value: "", Expires: time.Now()})
	w.WriteHeader(http.StatusOK)
}

func HandleSession(w http.ResponseWriter, r *http.Request) {
	id, s := GetSession(r)
	if s == nil {
		w.WriteHeader(http.StatusUnauthorized)
		return
	}
	sessionsSync.Lock()
	defer sessionsSync.Unlock()
	if s != nil {
		s.Expires = time.Now().Add(time.Minute * 5)
	}
	http.SetCookie(w, &http.Cookie{Name: SessionCookieName, HttpOnly: true, SameSite: http.SameSiteStrictMode, Value: id, Expires: s.Expires})
	w.WriteHeader(http.StatusOK)
	resp := SessionResponse{
		CommitHash: CommitHash,
	}
	res, err := json.Marshal(resp)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.Write(res)
}
