package mock

import (
	"context"

	billbot "github.com/jacobmichels/BillBot"
)

type MockBillReceiver struct {
	bills []billbot.Bill
}

func NewMockBillReceiver(bills []billbot.Bill) *MockBillReceiver {
	return &MockBillReceiver{bills}
}

func (mbr *MockBillReceiver) ReceiveBills(ctx context.Context) ([]billbot.Bill, error) {
	return mbr.bills, nil
}
