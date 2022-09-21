package redis

import (
	"context"
	"fmt"

	"github.com/go-redis/redis/v8"
	"github.com/rs/zerolog/log"
)

type Client struct {
	redis *redis.Client
}

func NewClient(addr, username, password string) *Client {
	redis := redis.NewClient(&redis.Options{
		Addr:     addr,
		Username: username,
		Password: password,
		OnConnect: func(ctx context.Context, cn *redis.Conn) error {
			log.Info().Msg("Redis client connected")
			return nil
		},
	})

	return &Client{redis}
}

func (c *Client) Ping(ctx context.Context) (string, error) {
	return c.redis.Ping(ctx).Result()
}

func (c *Client) EmailSeen(ctx context.Context, messageID string) (bool, error) {
	value, err := c.redis.Exists(ctx, messageID).Result()
	if err != nil {
		return false, fmt.Errorf("failed to check if key exists: %w", err)
	}

	return value == 1, nil
}

func (c *Client) SetEmailSeen(ctx context.Context, messageID string) error {
	_, err := c.redis.Set(ctx, messageID, true, 0).Result()
	if err != nil {
		return fmt.Errorf("failed to set email seen: %w", err)
	}

	return nil
}
