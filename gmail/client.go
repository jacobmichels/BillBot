package gmail

import (
	"bytes"
	"context"
	"encoding/base64"
	"fmt"
	"net/http"
	"net/mail"
	"os"
	"strings"

	billbot "github.com/jacobmichels/BillBot"
	"github.com/rs/zerolog/log"
	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
	"google.golang.org/api/gmail/v1"
	"google.golang.org/api/option"
)

type Client struct {
	gmailService   *gmail.Service
	filters        []billbot.EmailFilter
	seenMessageIDs map[string]struct{}
}

func NewGmailClient(ctx context.Context, credentialsFilePath, refreshToken string, filters []billbot.EmailFilter) (*Client, error) {
	b, err := os.ReadFile(credentialsFilePath)
	if err != nil {
		return nil, fmt.Errorf("failed to read credentials file: %w", err)
	}

	oauthConfig, err := google.ConfigFromJSON(b, gmail.GmailReadonlyScope)
	if err != nil {
		return nil, fmt.Errorf("failed to parse credentials file: %w", err)
	}
	http, err := createHttpClient(oauthConfig, refreshToken)
	if err != nil {
		return nil, fmt.Errorf("failed to create authenticated http client: %w", err)
	}

	service, err := gmail.NewService(ctx, option.WithHTTPClient(http))
	if err != nil {
		return nil, fmt.Errorf("failed to create gmail service: %w", err)
	}

	return &Client{service, filters, make(map[string]struct{})}, nil
}

func createHttpClient(oauthConfig *oauth2.Config, refreshToken string) (*http.Client, error) {
	token := &oauth2.Token{
		TokenType:    "Bearer",
		RefreshToken: refreshToken,
	}

	return oauthConfig.Client(context.Background(), token), nil
}

func (c *Client) PollBillEmails(ctx context.Context) ([]billbot.Bill, error) {
	listResponse, err := c.gmailService.Users.Messages.List("me").MaxResults(10).Do()
	if err != nil {
		return nil, fmt.Errorf("failed to list emails: %w", err)
	}

	var results []billbot.Bill
	for _, message := range listResponse.Messages {
		// if we haven't seen this message...
		if _, ok := c.seenMessageIDs[message.Id]; !ok {
			c.seenMessageIDs[message.Id] = struct{}{}

			getResponse, err := c.gmailService.Users.Messages.Get("me", message.Id).Format("RAW").Do()
			if err != nil {
				return nil, fmt.Errorf("failed to get email message %s: %w", message.Id, err)
			}

			raw, err := base64.URLEncoding.DecodeString(getResponse.Raw)
			if err != nil {
				panic(err)
			}

			// parse email into net/mail message
			msg, err := mail.ReadMessage(bytes.NewBuffer(raw))
			if err != nil {
				return nil, fmt.Errorf("failed to parse email: %w", err)
			}

			subject := msg.Header.Get("Subject")
			from := msg.Header.Get("From")

			for _, filter := range c.filters {
				if strings.Contains(subject, filter.Subject) && strings.Contains(from, filter.From) {
					log.Info().Str("type", filter.Name).Msg("Bill found")
					bill, err := filter.Parse(msg.Body)
					if err != nil {
						return nil, fmt.Errorf("failed to parse email body: %w", err)
					}

					results = append(results, bill)
				}
			}
		}
	}

	if len(results) > 0 {
		log.Info().Int("count", len(results)).Msg("saw new email(s)")
	}

	return results, nil
}
