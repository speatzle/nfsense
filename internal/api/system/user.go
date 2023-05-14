package system

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/auth"
	"nfsense.net/nfsense/internal/definitions/system"
)

type User struct {
	Comment string `json:"comment"`
}

type GetUserParameters struct {
	ID string
}

type GetUserResult struct {
	Name string `json:"name"`
	User
}

func (f *System) GetUser(ctx context.Context, params GetUserParameters) (GetUserResult, error) {
	_, ok := f.ConfigManager.GetPendingConfig().System.Users[params.ID]
	if !ok {
		return GetUserResult{}, fmt.Errorf("User does not Exist")
	}

	return GetUserResult{
		Name: params.ID,
		User: User{
			Comment: f.ConfigManager.GetPendingConfig().System.Users[params.ID].Comment,
		},
	}, nil
}

type GetUsersResult struct {
	Users map[string]User
}

func (f *System) GetUsers(ctx context.Context, params struct{}) (GetUsersResult, error) {
	users := map[string]User{}
	for n, u := range f.ConfigManager.GetPendingConfig().System.Users {
		users[n] = User{Comment: u.Comment}
	}
	return GetUsersResult{
		Users: users,
	}, nil
}

type CreateUserParameters struct {
	Name     string `json:"name"`
	Password string `json:"password"`
	User
}

func (f *System) CreateUser(ctx context.Context, params CreateUserParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().System.Users[params.Name]
	if ok {
		return struct{}{}, fmt.Errorf("User already Exists")
	}

	if params.Name == "" {
		return struct{}{}, fmt.Errorf("Name Cannot be empty")
	}

	if params.Password == "" {
		return struct{}{}, fmt.Errorf("Password Cannot be empty")
	}

	hash, salt, err := auth.GenerateHash(params.Password)
	if err != nil {
		return struct{}{}, fmt.Errorf("Generate Hash: %w", err)
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.System.Users[params.Name] = system.User{
		Hash:    hash,
		Salt:    salt,
		Comment: params.User.Comment,
	}
	return struct{}{}, t.Commit()
}

type UpdateUserParameters struct {
	Name     string `json:"name"`
	Password string `json:"password"`
	User
}

func (f *System) UpdateUser(ctx context.Context, params UpdateUserParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().System.Users[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("User does not Exist")
	}

	if params.Name == "" {
		return struct{}{}, fmt.Errorf("Name Cannot be empty")
	}
	user := f.ConfigManager.GetPendingConfig().System.Users[params.Name]

	if params.Password != "" {
		hash, salt, err := auth.GenerateHash(params.Password)
		if err != nil {
			return struct{}{}, fmt.Errorf("Generate Hash: %w", err)
		}

		user.Hash = hash
		user.Salt = salt
	}

	user.Comment = params.User.Comment

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.System.Users[params.Name] = user
	return struct{}{}, t.Commit()
}

type DeleteUserParameters struct {
	Name string
}

func (f *System) DeleteUser(ctx context.Context, params DeleteUserParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().System.Users[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("User does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	delete(conf.System.Users, params.Name)
	return struct{}{}, t.Commit()
}
