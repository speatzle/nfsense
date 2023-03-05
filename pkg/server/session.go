package server

import (
	"net/http"
	"sync"
	"time"

	"github.com/google/uuid"
)

type SessionKeyType string

const SessionKey SessionKeyType = "session"

type Session struct {
	Username string
	Expires  time.Time
}

var sessionsSync sync.Mutex
var sessions map[string]*Session = map[string]*Session{}

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
	http.SetCookie(w, &http.Cookie{Name: "session", HttpOnly: true, SameSite: http.SameSiteStrictMode, Value: id, Expires: expires})
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
	username := r.PostFormValue("username")
	password := r.PostFormValue("password")
	if username == "admin" && password == "12345" {
		GenerateSession(w, username)
		w.WriteHeader(http.StatusOK)
		http.Redirect(w, r, "/", http.StatusFound)
	}
	w.WriteHeader(http.StatusUnauthorized)
}

func HandleLogout(w http.ResponseWriter, r *http.Request) {
	http.SetCookie(w, &http.Cookie{Name: "session", Value: "", Expires: time.Now()})
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
	http.SetCookie(w, &http.Cookie{Name: "session", HttpOnly: true, SameSite: http.SameSiteStrictMode, Value: id, Expires: s.Expires})
	w.WriteHeader(http.StatusOK)
}
