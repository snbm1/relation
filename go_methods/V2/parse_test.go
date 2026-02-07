package V2

import (
	"os"
	"testing"
)

func TestParseFromCont(t *testing.T) {
	input := `{
      "log": { "level": "info" },
      "outbounds": []
    }`

	out, err := Parse(input, "")

	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}

	if out == "" {
		t.Fatal("Parsed config is empty")
	}
}

func TestParseFromFile(t *testing.T) {
	tmp, err := os.CreateTemp("", "cfg*.json")
	if err != nil {
		t.Fatal(err)
	}

	defer os.Remove(tmp.Name())

	_, _ = tmp.WriteString(`{"outbounds":[]}`)
	tmp.Close()

	out, err := Parse("", tmp.Name())

	if err != nil {
		t.Fatalf("Parse from file failed: %v", err)
	}

	if out == "" {
		t.Fatal("Parsed config is empty")
	}
}
