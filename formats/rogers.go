package formats

import (
	"fmt"
	"io"
	"strings"
	"time"

	"github.com/antchfx/htmlquery"
	"github.com/bojanz/currency"
	billbot "github.com/jacobmichels/BillBot"
)

type RogersEmailParser struct {
}

func (rep *RogersEmailParser) Parse(body io.Reader) (billbot.Bill, error) {
	var result billbot.Bill

	doc, err := htmlquery.Parse(body)
	if err != nil {
		return result, fmt.Errorf("failed to parse rogers email html: %w", err)
	}

	parent, err := htmlquery.Query(doc, "/html/body/table/tbody/tr/td/table/tbody/tr/td/table[5]/tbody/tr/td[2]/table/tbody/tr/td[2]")
	if err != nil {
		return result, fmt.Errorf("xpath query failed: %w", err)
	}

	amountStr := strings.TrimRight(strings.Split(parent.FirstChild.NextSibling.NextSibling.Data, "$")[1], " \t\n")
	amount, err := currency.NewAmount(amountStr, "CAD")
	if err != nil {
		return result, fmt.Errorf("failed to parse amount: %w", err)
	}

	result.Name = "Rogers internet"
	result.TotalAmount = amount
	result.ReceivedDate = time.Now().String()

	return result, nil
}
