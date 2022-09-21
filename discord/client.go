package discord

import (
	"context"
	"fmt"

	"github.com/bwmarrin/discordgo"
	"github.com/rs/zerolog/log"
)

type Client struct {
	session        *discordgo.Session
	channelId      string
	serviceOwnerID string
}

func NewClient(token, channelID, serviceOwnerID string) (*Client, error) {
	session, err := discordgo.New("Bot " + token)
	if err != nil {
		return nil, fmt.Errorf("failed to create discordgo session: %w", err)
	}

	session.AddHandler(func(s *discordgo.Session, r *discordgo.Ready) {
		log.Info().Msg("Discordgo session ready")
	})

	if err = session.Open(); err != nil {
		return nil, fmt.Errorf("failed to open discordgo session: %w", err)
	}

	return &Client{
		session,
		channelID,
		serviceOwnerID,
	}, nil
}

func (c *Client) SendMessage(ctx context.Context, message string) error {
	_, err := c.session.ChannelMessageSend(c.channelId, message)
	if err != nil {
		return fmt.Errorf("failed to send message: %w", err)
	}
	return nil
}

func (c *Client) AlertServiceOwner(ctx context.Context, message string) error {
	channel, err := c.session.UserChannelCreate(c.serviceOwnerID)
	if err != nil {
		return fmt.Errorf("failed to create service owner channel: %w", err)
	}

	_, err = c.session.ChannelMessageSend(channel.ID, message)
	if err != nil {
		return fmt.Errorf("failed to send channel message: %w", err)
	}

	return nil
}

func (c *Client) Close() error {
	return c.session.Close()
}
