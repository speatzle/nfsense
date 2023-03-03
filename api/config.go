package main

import (
	"encoding/json"
	"fmt"
	"os"

	"nfsense.net/nfsense/pkg/definitions"
)

func LoadConfiguration(file string) (*definitions.Config, error) {
	var config definitions.Config
	configFile, err := os.Open(file)
	if err != nil {
		return nil, fmt.Errorf("opening Config File %w", err)
	}
	defer configFile.Close()
	if err != nil {
		fmt.Println(err.Error())
	}
	jsonParser := json.NewDecoder(configFile)
	jsonParser.DisallowUnknownFields()
	err = jsonParser.Decode(&config)
	if err != nil {
		return nil, fmt.Errorf("decoding Config File %w", err)
	}
	return &config, nil
}
