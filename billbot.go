package billbot

import (
	"context"
	"io"

	"github.com/bojanz/currency"
)

type Bill struct {
	Name         string
	ReceivedDate string
	TotalAmount  currency.Amount
}

type BackgroundService interface {
	Start(context.Context) error
}

type BillSender interface {
	SendBill(context.Context, Bill) error
	Close() error
}

type BillReceiver interface {
	ReceiveBills(context.Context) ([]Bill, error)
}

type DiscordClient interface {
	SendMessage(context.Context, string) error
	AlertServiceOwner(context.Context, string) error
	Close() error
}

type EmailFilter struct {
	Name    string
	From    string
	Subject string
	EmailParser
}

type EmailParser interface {
	Parse(io.Reader) (Bill, error)
}

type GmailClient interface {
	PollBillEmails(context.Context) ([]Bill, error)
}
