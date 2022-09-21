package mock

import (
	"context"

	billbot "github.com/jacobmichels/BillBot"
)

type MockBillSender struct {
	BillsSent chan billbot.Bill
}

func NewMockBillSender() *MockBillSender {
	return &MockBillSender{BillsSent: make(chan billbot.Bill, 10)}
}

func (mbs *MockBillSender) SendBill(ctx context.Context, bill billbot.Bill) error {
	mbs.BillsSent <- bill

	return nil
}

func (mbs *MockBillSender) Close() error {
	return nil
}
