package publisher_test

import (
	"context"
	"testing"
	"time"

	"github.com/bojanz/currency"
	billbot "github.com/jacobmichels/BillBot"
	"github.com/jacobmichels/BillBot/mock"
	"github.com/jacobmichels/BillBot/publisher"
	"github.com/stretchr/testify/require"
)

func TestStartCancelled(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Millisecond*500)
	defer cancel()
	bills := make(chan billbot.Bill, 10)
	mockSender := mock.NewMockBillSender()

	pub := publisher.NewBillPublisher(bills, mockSender)
	err := pub.Start(ctx)
	require.NoError(t, err)
	close(mockSender.BillsSent)

	billsSentCount := 0
	for range mockSender.BillsSent {
		billsSentCount++
	}

	require.Equal(t, billsSentCount, 0)
}

func TestStart(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Millisecond*500)
	defer cancel()
	bills := make(chan billbot.Bill, 10)
	for i := 0; i < 10; i++ {
		bills <- billbot.Bill{Name: "test", ReceivedDate: "test", TotalAmount: currency.Amount{}}
	}
	mockSender := mock.NewMockBillSender()

	pub := publisher.NewBillPublisher(bills, mockSender)
	err := pub.Start(ctx)
	require.NoError(t, err)
	close(mockSender.BillsSent)

	billsSentCount := 0
	for range mockSender.BillsSent {
		billsSentCount++
	}

	require.Equal(t, billsSentCount, 10)
}
