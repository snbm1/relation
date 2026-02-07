package V2

import (
	"os"
	"testing"
)

func writeTempConfig(t *testing.T, content string) string {
	t.Helper()

	tmp, err := os.CreateTemp("", "sub-*.json")
	if err != nil {
		t.Fatal(err)
	}

	_, err = tmp.WriteString(content)
	if err != nil {
		t.Fatal(err)
	}

	tmp.Close()
	return tmp.Name()
}

func TestStart(t *testing.T) {
	cfg := writeTempConfig(t, `{
		"log": { "level": "error" },
		"inbounds": [{ "type": "direct", "tag": "in" }],
		"outbounds": [{ "type": "direct", "tag": "direct" }]
	}`)

	defer os.Remove(cfg)
	defer Stop()

	err := Start(cfg, false)
	if err != nil {
		t.Fatalf("Start failed: %v", err)
	}

	if Box == nil {
		t.Fatal("Box is nil after start method")
	}
}

func TestStop(t *testing.T) {
	cfg := writeTempConfig(t, `{
		"log": { "level": "error" },
		"inbounds": [{ "type": "direct", "tag": "in" }],
		"outbounds": [{ "type": "direct", "tag": "direct" }]
	}`)

	defer os.Remove(cfg)

	if err := Start(cfg, false); err != nil {
		t.Fatalf("Start failed: %v", err)
	}

	err := Stop()
	if err != nil {
		t.Fatalf("Stop failed: %v", err)
	}

	if Box != nil {
		t.Fatal("Box not nil after Stop method")
	}
}

func TestRestart(t *testing.T) {
	cfg := writeTempConfig(t, `{
		"log": { "level": "error" },
		"inbounds": [{ "type": "direct", "tag": "in" }],
		"outbounds": [{ "type": "direct", "tag": "direct" }]
	}`)

	defer os.Remove(cfg)
	defer Stop()

	if err := Start(cfg, false); err != nil {
		t.Fatalf("Start failed: %v", err)
	}

	oldBox := Box

	err := Restart(cfg, false)
	if err != nil {
		t.Fatalf("Restart failed: %v", err)
	}

	if Box == nil {
		t.Fatal("Box is nil after Restart method")
	}

	if Box == oldBox {
		t.Fatal("Restart did not create new Box instance")
	}
}
