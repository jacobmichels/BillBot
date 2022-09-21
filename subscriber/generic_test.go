package subscriber_test

import (
	"context"
	"testing"
	"time"

	billbot "github.com/jacobmichels/BillBot"
	"github.com/jacobmichels/BillBot/mock"
	"github.com/jacobmichels/BillBot/subscriber"
	"github.com/stretchr/testify/require"
)

func TestStartCancelled(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Millisecond*500)
	defer cancel()
	bills := make(chan billbot.Bill, 10)
	mockReceiver := mock.NewMockBillReceiver([]billbot.Bill{})
	sub := subscriber.NewBillSubscriber(bills, mockReceiver)

	err := sub.Start(ctx)
	require.NoError(t, err)
	close(bills)

	billsCount := 0
	for range bills {
		billsCount++
	}

	require.Equal(t, billsCount, 0)
}

func TestStart(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Millisecond*500)
	defer cancel()
	bills := make(chan billbot.Bill, 10)
	mockReceiver := mock.NewMockBillReceiver([]billbot.Bill{
		{}, {}, {},
	})
	sub := subscriber.NewBillSubscriber(bills, mockReceiver)

	err := sub.Start(ctx)
	require.NoError(t, err)
	close(bills)

	billsCount := 0
	for range bills {
		billsCount++
	}

	require.Equal(t, billsCount, 3)
}
