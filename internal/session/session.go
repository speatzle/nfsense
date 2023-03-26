package session

import (
	"net/http"
	"runtime/debug"
	"sync"
	"time"

	"github.com/google/uuid"
)

type SessionKeyType string

const SessionKey SessionKeyType = "session"
const SessionCookieName string = "session"
const SessionLifeTime = time.Minute * 15

type Session struct {
	Username string
	Expires  time.Time
	// TODO Add []websocket.Conn pointer to close all active websockets, alternativly do this via context cancelation
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

func ExtendSession(s *Session) {
	sessionsSync.Lock()
	defer sessionsSync.Unlock()
	if s != nil {
		s.Expires = time.Now().Add(SessionLifeTime)
	}
}

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
	expires := time.Now().Add(SessionLifeTime)
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
