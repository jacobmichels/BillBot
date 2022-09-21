package main

import (
	"fmt"
	"strings"

	"github.com/spf13/viper"
)

type Config struct {
	Discord struct {
		Token          string `mapstructure:"token"`
		ChannelID      string `mapstructure:"channel_id"`
		ServiceOwnerID string `mapstructure:"service_owner_id"`
	}
	Log struct {
		Pretty bool `mapstructure:"pretty"`
	}
	Gmail struct {
		CredentialsFilePath string `mapstructure:"credentials_file"`
		RefreshToken        string `mapstructure:"refresh_token"`
	}
	Redis struct {
		Addr     string `mapstructure:"addr"`
		Username string `mapstructure:"username"`
		Password string `mapstructure:"password"`
	}
}

func ReadConfig() (Config, bool, error) {
	viper.AddConfigPath(".")
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")

	viper.SetDefault("discord.token", "")
	viper.SetDefault("discord.channel_id", "")
	viper.SetDefault("discord.service_owner_id", "")
	viper.SetDefault("log.pretty", "")
	viper.SetDefault("gmail.credentials_file", "")
	viper.SetDefault("gmail.refresh_token", "")
	viper.SetDefault("redis.addr", "")
	viper.SetDefault("redis.username", "")
	viper.SetDefault("redis.password", "")

	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	viper.AutomaticEnv()

	var fileFound bool = true
	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); ok {
			fileFound = false
		} else {
			// Config file was found but another error was produced
			return Config{}, fileFound, fmt.Errorf("failed to read config file: %s", err)
		}
	}

	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		return Config{}, fileFound, fmt.Errorf("failed to unmarshal config: %s", err)
	}

	return config, fileFound, nil
}
