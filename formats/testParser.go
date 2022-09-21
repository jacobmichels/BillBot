package formats

import (
	"io"
	"time"

	"github.com/bojanz/currency"
	billbot "github.com/jacobmichels/BillBot"
)

type TestEmailParser struct {
}

func (tep *TestEmailParser) Parse(body io.Reader) (billbot.Bill, error) {
	amount, _ := currency.NewAmount("25.65", "CAD")
	return billbot.Bill{Name: "test bill", ReceivedDate: time.Now().String(), TotalAmount: amount}, nil
}
