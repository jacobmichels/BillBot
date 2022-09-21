package receiver

import (
	"context"
	"fmt"

	billbot "github.com/jacobmichels/BillBot"
)

type GmailReceiver struct {
	client billbot.GmailClient
}

func NewGmailReceiver(client billbot.GmailClient) *GmailReceiver {
	return &GmailReceiver{client}
}

func (gmr *GmailReceiver) ReceiveBills(ctx context.Context) ([]billbot.Bill, error) {
	bills, err := gmr.client.PollBillEmails()
	if err != nil {
		return nil, fmt.Errorf("failed to list bill emails: %w", err)
	}

	return bills, nil
}
