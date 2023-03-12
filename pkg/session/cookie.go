package session

import (
	"net/http"
	"time"
)

func GetCookie(value string, expires time.Time) *http.Cookie {
	return &http.Cookie{Name: SessionCookieName, HttpOnly: true, SameSite: http.SameSiteStrictMode, Value: value, Expires: expires}
}
