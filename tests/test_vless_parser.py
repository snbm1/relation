import unittest

from vless_parser import parse_vless_url


class VlessParserTests(unittest.TestCase):
    def test_minimal_url(self) -> None:
        parsed = parse_vless_url("vless://user@example.com")

        self.assertEqual(parsed["type"], "vless")
        self.assertEqual(parsed["server"], "example.com")
        self.assertEqual(parsed["server_port"], 443)
        self.assertEqual(parsed["uuid"], "user")
        self.assertEqual(parsed["tag"], "vless_outbound")
        self.assertEqual(parsed["packet_encoding"], "xudp")

    def test_ws_tls_url(self) -> None:
        parsed = parse_vless_url(
            "vless://user@example.com:8443"
            "?type=ws&security=tls&sni=cdn.example.com&host=site.example.com&path=%2Fws"
            "#demo"
        )

        self.assertEqual(parsed["tag"], "demo")
        self.assertEqual(parsed["tls"]["server_name"], "cdn.example.com")
        self.assertTrue(parsed["tls"]["insecure"])
        self.assertEqual(parsed["transport"]["type"], "ws")
        self.assertEqual(parsed["transport"]["path"], "/ws")
        self.assertEqual(parsed["transport"]["headers"]["Host"], "site.example.com")

    def test_reality_requires_keys(self) -> None:
        with self.assertRaises(ValueError):
            parse_vless_url(
                "vless://user@example.com?security=reality&sni=cdn.example.com"
            )


if __name__ == "__main__":
    unittest.main()
