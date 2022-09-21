package main

import (
	"context"
	"fmt"
	"os"

	billbot "github.com/jacobmichels/BillBot"
	"github.com/jacobmichels/BillBot/discord"
	"github.com/jacobmichels/BillBot/formats"
	"github.com/jacobmichels/BillBot/gmail"
	"github.com/jacobmichels/BillBot/publisher"
	"github.com/jacobmichels/BillBot/receiver"
	"github.com/jacobmichels/BillBot/sender"
	"github.com/jacobmichels/BillBot/subscriber"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	ctx := context.Background()

	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr})

	cfg, ok, err := ReadConfig()
	if err != nil {
		log.Panic().Err(err).Msg("failed to read config")
	}
	if !ok {
		log.Info().Msg("No config file found, using env and defaults")
	}

	// create the dependencies for the main services
	discordClient, err := discord.NewClient(cfg.Discord.Token, cfg.Discord.ChannelID, cfg.Discord.ServiceOwnerID)
	if err != nil {
		log.Fatal().Err(err).Msg("failed to create discord client")
	}
	defer discordClient.Close()

	discordSender := sender.NewDiscordBillSender(discordClient)

	filters := []billbot.EmailFilter{
		{
			Name:        "Rogers internet",
			From:        "notifications@rci.rogers.com",
			Subject:     "Your Rogers bill is now available",
			EmailParser: &formats.RogersEmailParser{},
		},
		{
			Name:        "Test",
			From:        "jacob.michels2025@gmail.com",
			Subject:     "Test",
			EmailParser: &formats.TestEmailParser{},
		},
	}

	gmailClient, err := gmail.NewGmailClient(ctx, cfg.Gmail.CredentialsFilePath, cfg.Gmail.RefreshToken, filters)
	if err != nil {
		log.Fatal().Err(err).Msg("failed to create gmail client")
	}

	recv := receiver.NewGmailReceiver(gmailClient)

	// create the bills channel
	bills := make(chan billbot.Bill, 10)

	// create the bill subscriber
	// this is what listens for incoming bills
	sub := subscriber.NewBillSubscriber(bills, recv)

	// create a notification publisher
	// this is what receives bills and notifies the users of them
	pub := publisher.NewBillPublisher(bills, discordSender)

	// TODO cancel context when interrupt signal caught

	if err = run(ctx, sub, pub); err != nil {
		log.Fatal().Err(err).Msg("an error occured running BillBot")
	}
}

func run(ctx context.Context, sub billbot.BackgroundService, pub billbot.BackgroundService) error {
	errChan := make(chan error)

	go func() {
		log.Info().Msg("Starting publisher")
		if err := pub.Start(ctx); err != nil {
			errChan <- fmt.Errorf("publisher failed: %w", err)
		}
	}()

	go func() {
		log.Info().Msg("Starting subscriber")
		if err := sub.Start(ctx); err != nil {
			errChan <- fmt.Errorf("subscriber failed :%w", err)
		}
	}()

	select {
	case <-ctx.Done():
		return nil
	case err := <-errChan:
		return err
	}
}
