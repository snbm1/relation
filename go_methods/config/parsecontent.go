package config

import (
	"bytes"
	_ "embed"
	"encoding/json"
	"fmt"

	"github.com/ghodss/yaml"
	"github.com/hiddify/ray2sing/ray2sing"
	"github.com/sagernet/sing-box/experimental/libbox"
	"github.com/xmdhs/clash2singbox/convert"
	"github.com/xmdhs/clash2singbox/model/clash"
)

//go:embed config.json.template
var confByte []byte

func ConfigParser(contentstr string) ([]byte, error) {
	content := []byte(contentstr)
	var tmp any

	decoder := json.NewDecoder(bytes.NewReader(content))
	if err := decoder.Decode(&tmp); err == nil {
		normal, err := json.MarshalIndent(tmp, "", " ")
		if err != nil {
			return nil, err
		}
		return validateResult(normal, "SingBoxJson")
	}

	if v2raystr, err := ray2sing.Ray2Singbox(string(content), false); err == nil {
		return validateResult([]byte(v2raystr), "V2ray")
	}

	var clashobj clash.Clash
	if err := yaml.Unmarshal(content, &clashobj); err == nil && clashobj.Proxies != nil {
		if len(clashobj.Proxies) == 0 {
			return nil, fmt.Errorf("no outbounds found")
		}

		converted, err := convert.Clash2sing(clashobj)
		if err != nil {
			return nil, err
		}

		output := append([]byte(nil), confByte...)
		output, err = convert.Patch(output, converted, "", "", nil)

		if err != nil {
			return nil, err
		}
		return validateResult(output, "Clash")
	}

	return nil, fmt.Errorf("unsupported config format")
}

func validateResult(content []byte, name string) ([]byte, error) {
	err := libbox.CheckConfig(string(content))
	if err != nil {
		return nil, fmt.Errorf("[%s] invalid sing-box config: %w", name, err)
	}

	return content, nil
}
