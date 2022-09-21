package subscriber

import (
	"context"
	"fmt"
	"time"

	billbot "github.com/jacobmichels/BillBot"
	"github.com/rs/zerolog/log"
)

type BillSubscriber struct {
	bills    chan<- billbot.Bill
	receiver billbot.BillReceiver
}

// Subscriber takes a send-only channel for bills
func NewBillSubscriber(bills chan<- billbot.Bill, receiver billbot.BillReceiver) *BillSubscriber {
	return &BillSubscriber{bills, receiver}
}

func (s *BillSubscriber) Start(ctx context.Context) error {
	receiveChan := make(chan billbot.Bill)
	errChan := make(chan error)
	go func() {
		for {
			bills, err := s.receiver.ReceiveBills(ctx)
			if err != nil {
				errChan <- fmt.Errorf("failed to receive bill: %w", err)
				return
			}
			for _, bill := range bills {
				log.Info().Msg("Subscriber received bills, pushing to channel")
				receiveChan <- bill
			}

			select {
			case <-ctx.Done():
				log.Info().Msg("subscriber shut down")
				return
			case <-time.After(time.Minute):
				// continue to next iteration
			}
		}

	}()

	for {
		select {
		case bill := <-receiveChan:
			s.bills <- bill
		case <-ctx.Done():
			return nil
		}
	}
}
