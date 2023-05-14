package auth

import (
	"fmt"
	"math/rand"
	"time"

	"github.com/tredoe/osutil/user/crypt/sha512_crypt"
	"nfsense.net/nfsense/internal/definitions/config"
)

func AuthenticateUser(conf config.Config, username, password string) error {
	user, ok := conf.System.Users[username]
	if !ok {
		return fmt.Errorf("User not found")
	}

	// Using sha512 to be compatible with /etc/shadow
	c := sha512_crypt.New()
	hash, err := c.Generate([]byte(password), []byte(user.Salt))
	if err != nil {
		return fmt.Errorf("Hashing Password: %w", err)
	}

	if hash == user.Hash {
		return nil
	}

	return fmt.Errorf("Invalid Password")
}

func GenerateHash(password string) (string, string, error) {
	const charset = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	seededRand := rand.New(rand.NewSource(time.Now().UnixNano()))
	s := make([]byte, 8)
	for i := range s {
		s[i] = charset[seededRand.Intn(len(charset))]
	}
	salt := []byte(fmt.Sprintf("$6$%s", s))

	c := sha512_crypt.New()
	hash, err := c.Generate([]byte(password), []byte(salt))
	if err != nil {
		return "", "", fmt.Errorf("Hashing Password: %w", err)
	}
	return hash, string(salt), nil
}
