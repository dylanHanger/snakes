package images

import (
	_ "embed"
)

//go:embed default.png
var DefaultAtlas []byte

//go:embed legacy.png
var LegacyAtlas []byte
