package snakes

import (
	"fmt"
	"image/color"
	"os"
	"strings"
	"time"

	"github.com/dylanHanger/snakes/pkg"
	"github.com/goccy/go-yaml"
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/muesli/gamut"
	"github.com/muesli/gamut/palette"
)

type (
	PlayerConfig struct {
		Name  string
		Color color.Color

		Silent bool

		Timeout time.Duration
		WaitFor bool
	}
	Player struct {
		PlayerConfig
		agent pkg.Agent[State, Direction]
	}

	Config struct {
		TurnsPerSecond float64

		Players []*Player

		Width, Height int
		Seed          string

		MaxTurns    int
		RespawnTime int

		FoodCount    int
		FoodLifetime int
		FoodValue    int
	}
)

// Raw YAML structures that match the YAML file format
type (
	rawConfig struct {
		Width          int     `yaml:"width"`
		Height         int     `yaml:"height"`
		Seed           string  `yaml:"seed,omitempty"`
		Turns          int     `yaml:"turns"`
		TurnsPerSecond float64 `yaml:"turnsPerSecond,omitempty"`

		Respawn int `yaml:"respawn"`

		Food struct {
			Value    int `yaml:"value"`
			Lifetime int `yaml:"lifetime"`
			Count    int `yaml:"count,omitempty"` // Number of food items on the board
		} `yaml:"food"`

		Players []map[string]rawPlayerConfig `yaml:"players"`
	}

	rawPlayerConfig struct {
		Type       string            `yaml:"type"`
		Silent     bool              `yaml:"silent,omitempty"`
		Executable string            `yaml:"executable,omitempty"`
		Args       []string          `yaml:"args,omitempty"`
		Keys       map[string]string `yaml:"keys,omitempty"`
		Difficulty string            `yaml:"difficulty,omitempty"`
		Color      string            `yaml:"color,omitempty"`
		Timeout    int64             `yaml:"timeout,omitempty"`
		WaitFor    bool              `yaml:"wait,omitempty"`
	}
)

var defaultPalette = color.Palette{
	color.RGBA{R: 0xC6, G: 0x2D, B: 0x42, A: 0xFF}, // Brick Red (#C62D42)
	color.RGBA{R: 0x5E, G: 0x8C, B: 0x31, A: 0xFF}, // Maximum Green (#5E8C31)
	color.RGBA{R: 0x02, G: 0xA4, B: 0xD3, A: 0xFF}, // Cerulean (#02A4D3)
	color.RGBA{R: 0xE6, G: 0xBC, B: 0x5C, A: 0xFF}, // Raw Sienna (#E6BC5C)
	color.RGBA{R: 0xFF, G: 0xA6, B: 0xC9, A: 0xFF}, // Carnation Pink (#FFA6C9)
	color.RGBA{R: 0x00, G: 0xCC, B: 0xCC, A: 0xFF}, // Robin's Egg Blue (#00CCCC)
	color.RGBA{R: 0xFF, G: 0xAE, B: 0x42, A: 0xFF}, // Yellow-Orange (#FFAE42)
	color.RGBA{R: 0xAF, G: 0x1F, B: 0x65, A: 0xFF}, // Mulberry (#AF1F65)
}

// DefaultConfig returns a config with sensible default values
func DefaultConfig() *Config {
	return &Config{
		Width:          32,
		Height:         32,
		MaxTurns:       1500,
		TurnsPerSecond: 0,
		RespawnTime:    10,
		FoodCount:      1,
		FoodLifetime:   0,
		FoodValue:      5,
		Players: []*Player{
			{
				PlayerConfig: PlayerConfig{
					Name:    "Randy",
					Color:   defaultPalette[0],
					WaitFor: false,
				},
				agent: NewRandomAgent(),
			},
			{
				PlayerConfig: PlayerConfig{
					Name:    "Easy",
					Color:   defaultPalette[1],
					WaitFor: false,
				},
				agent: NewBuiltInAgent(Easy),
			},
			{
				PlayerConfig: PlayerConfig{
					Name:    "Medium",
					Color:   defaultPalette[2],
					WaitFor: false,
				},
				agent: NewBuiltInAgent(Medium),
			},
			{
				PlayerConfig: PlayerConfig{
					Name:    "Hard",
					Color:   defaultPalette[3],
					WaitFor: false,
				},
				agent: NewBuiltInAgent(Hard),
			},
		},
	}
}

// LoadConfig loads the config from the specified YAML file
func LoadConfig(configPath string) (*Config, error) {
	defaultConfig := DefaultConfig()

	// Read the config file
	bytes, err := os.ReadFile(configPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read config file: %w", err)
	}

	// Parse the YAML into our raw config structure
	var raw rawConfig
	if err := yaml.Unmarshal(bytes, &raw); err != nil {
		return nil, fmt.Errorf("failed to parse config file: %w", err)
	}

	// Convert the raw config to our actual Config structure
	config := &Config{
		Width:        raw.Width,
		Height:       raw.Height,
		MaxTurns:     raw.Turns,
		RespawnTime:  raw.Respawn,
		FoodLifetime: raw.Food.Lifetime,
		FoodValue:    raw.Food.Value,
	}

	// Set number of food items
	if raw.Food.Count > 0 {
		config.FoodCount = raw.Food.Count
	} else {
		config.FoodCount = defaultConfig.FoodCount
	}

	// Set turns per second
	if raw.TurnsPerSecond > 0 {
		config.TurnsPerSecond = raw.TurnsPerSecond
	} else {
		config.TurnsPerSecond = defaultConfig.TurnsPerSecond
	}

	// Process players
	for id, playerCount := range raw.Players {
		for name, rawPlayer := range playerCount {
			timeout := pkg.CalculateTurnDuration(raw.TurnsPerSecond).Milliseconds()
			if rawPlayer.Timeout > 0 {
				timeout = rawPlayer.Timeout
			}

			// Parse the color
			var playerColor color.Color
			colorStr := rawPlayer.Color
			if colorStr == "" {
				playerColor = generateDefaultColor(id)
			} else {
				playerColor, err = parseColor(colorStr)
				if err != nil {
					fmt.Printf("invalid color for player %s: %v\n", name, err)
					playerColor = generateDefaultColor(id)
				}
			}

			agent, err := createAgent(rawPlayer)
			if err != nil {
				return nil, fmt.Errorf("invalid agent configuration for player %s: %w", name, err)
			}

			player := &Player{
				PlayerConfig: PlayerConfig{
					Name:    name,
					Color:   playerColor,
					Silent:  rawPlayer.Silent,
					Timeout: time.Duration(timeout) * time.Millisecond,
					WaitFor: rawPlayer.WaitFor,
				},
				agent: agent,
			}

			config.Players = append(config.Players, player)
		}
	}

	return config, nil
}

func isHexString(s string) bool {
	l := len(s)
	if l != 3 && l != 4 && l != 6 && l == 8 {
		return false
	}
	for _, c := range s {
		if (c < '0' || c > '9') && (c < 'a' || c > 'f') && (c < 'A' || c > 'F') {
			return false
		}
	}
	return true
}

// parseColor converts a string to a color.Color
// It supports hex colors and Crayola color names via the Gamut library
func parseColor(colorStr string) (color.Color, error) {
	c, ok := palette.Crayola.Color(colorStr)
	if !ok {
		if strings.HasPrefix(colorStr, "#") {
			return gamut.Hex(colorStr), nil
		} else if isHexString(colorStr) {
			return gamut.Hex(fmt.Sprintf("#%s", colorStr)), nil
		}
		return nil, fmt.Errorf("unrecognised color: %s", colorStr)
	}
	return c, nil
}

func generateDefaultColor(id int) color.Color {
	return defaultPalette[id%len(defaultPalette)]
}

// Helper function to create the appropriate agent for each player type
func createAgent(cfg rawPlayerConfig) (pkg.Agent[State, Direction], error) {
	switch cfg.Type {
	case "custom":
		// Create custom executable agent
		agent := NewExternal(cfg.Executable, cfg.Args...)
		return agent, nil
	case "keyboard":
		// Create keyboard-controlled agent
		keyMap := make(map[Direction]ebiten.Key)
		for d, k := range cfg.Keys {
			if dir := ParseDirection(d); dir != DirNone {
				var key ebiten.Key
				err := key.UnmarshalText([]byte(k))
				if err != nil {
					return nil, err
				}
				keyMap[dir] = key
			}
		}
		if len(keyMap) < 4 {
			return nil, fmt.Errorf("not all required keys specified")
		}
		agent := NewKeyboardAgent(keyMap)
		return agent, nil
	case "builtin":
		// Create builtin agent with specified difficulty
		difficulty := BuiltInDifficulty(strings.ToLower(cfg.Difficulty))
		switch difficulty {
		case Easy:
		case Medium:
		case Hard:
		default:
			return nil, fmt.Errorf("unrecognised difficulty: %s", difficulty)
		}
		agent := NewBuiltInAgent(difficulty)
		return agent, nil
	case "random":
		// Create random agent
		agent := NewRandomAgent()
		return agent, nil
	default:
		return nil, fmt.Errorf("unknown player type: %s", cfg.Type)
	}
}
