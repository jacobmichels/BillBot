package session

import (
	"fmt"

	"github.com/bwmarrin/discordgo"
)

type DiscordgoSession struct {
	*discordgo.Session
}

func New(token string) (*DiscordgoSession, error) {
	wrapped, err := discordgo.New("Bot " + token)
	if err != nil {
		return nil, fmt.Errorf("failed to create discordgo session: %w", err)
	}

	return &DiscordgoSession{wrapped}, nil
}
