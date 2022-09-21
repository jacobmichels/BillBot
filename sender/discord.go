package sender

import (
	"context"
	"fmt"

	billbot "github.com/jacobmichels/BillBot"
)

type DiscordBillSender struct {
	client billbot.DiscordClient
}

func NewDiscordBillSender(client billbot.DiscordClient) *DiscordBillSender {
	return &DiscordBillSender{client}
}

func (dbs *DiscordBillSender) SendBill(ctx context.Context, bill billbot.Bill) error {
	// Convert the bill into a message string, use discord client to send the message
	split, err := bill.TotalAmount.Div("4")
	if err != nil {
		return fmt.Errorf("failed to split total amount: %w", err)
	}
	return dbs.client.SendMessage(ctx, fmt.Sprintf("**🚨 AYO NEW BILL AVAILABLE 🚨**\n >>> Kind: %s\nTotal amount: %s\nSplit four ways: %s\n\nPlease etransfer __jacob.michels2025@gmail.com__ as soon as possible. *Thanks lads ❤️*", bill.Name, bill.TotalAmount, split))
}

func (dbs *DiscordBillSender) Close() error {
	return nil
}
