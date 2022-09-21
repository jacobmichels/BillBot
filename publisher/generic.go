package publisher

import (
	"context"
	"fmt"

	billbot "github.com/jacobmichels/BillBot"
	"github.com/rs/zerolog/log"
)

type BillPublisher struct {
	bills  <-chan billbot.Bill
	sender billbot.BillSender
}

// Publisher takes a receive-only channel for bills
func NewBillPublisher(bills <-chan billbot.Bill, sender billbot.BillSender) *BillPublisher {
	return &BillPublisher{bills, sender}
}

// Start reading from the bills channel, forwarding all to the BillSender
func (p *BillPublisher) Start(ctx context.Context) error {
	for {
		select {
		case bill := <-p.bills:
			log.Info().Msg("Publisher sending bill")
			if err := p.sender.SendBill(ctx, bill); err != nil {
				return fmt.Errorf("failed to send bill: %w", err)
			}
		case <-ctx.Done():
			defer p.sender.Close()
			log.Info().Msg("Publisher shut down")
			return nil
		}
	}
}
