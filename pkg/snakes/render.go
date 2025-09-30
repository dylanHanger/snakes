package snakes

import (
	"bytes"
	"fmt"
	"image"
	"image/color"
	"math"
	"slices"
	"strings"

	_ "image/png"

	images "github.com/dylanHanger/snakes/pkg/snakes/resources"
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
	"github.com/hajimehoshi/ebiten/v2/vector"
	"github.com/muesli/gamut"
)

// drawGridCell draws a cell at the specified grid position
func (g *Game) drawGridCell(screen *ebiten.Image, p GridPoint, scale float32, color color.Color) {
	screenSize := screen.Bounds().Size()
	screenWidth := float32(screenSize.X)
	screenHeight := float32(screenSize.Y)

	gameWidth := float32(g.config.Width)
	gameHeight := float32(g.config.Height)

	// Calculate cell size to maintain square aspect ratio
	cellSize := min(screenWidth/gameWidth, screenHeight/gameHeight)

	// Calculate grid dimensions
	gridWidth := cellSize * gameWidth
	gridHeight := cellSize * gameHeight

	// Calculate offsets to center the grid on screen
	offsetX := (screenWidth - gridWidth) / 2
	offsetY := (screenHeight - gridHeight) / 2

	// Calculate position for this cell
	posX := offsetX + float32(p.X)*cellSize
	posY := offsetY + float32(p.Y)*cellSize

	// Calculate the inner cell size based on scale
	cellInnerSize := scale * cellSize
	cellInnerOffset := (cellSize - cellInnerSize) / 2

	vector.DrawFilledRect(
		screen,
		posX+cellInnerOffset,
		posY+cellInnerOffset,
		cellInnerSize,
		cellInnerSize,
		color,
		false,
	)
}

func (g *Game) drawGridCellImage(screen *ebiten.Image, img *ebiten.Image, p GridPoint, rotations int) {
	screenSize := screen.Bounds().Size()
	screenWidth := float32(screenSize.X)
	screenHeight := float32(screenSize.Y)

	gameWidth := float32(g.config.Width)
	gameHeight := float32(g.config.Height)

	// Calculate cell size to maintain square aspect ratio
	cellSize := min(screenWidth/gameWidth, screenHeight/gameHeight)

	// Calculate grid dimensions
	gridWidth := cellSize * gameWidth
	gridHeight := cellSize * gameHeight

	// Calculate offsets to center the grid on screen
	offsetX := (screenWidth - gridWidth) / 2
	offsetY := (screenHeight - gridHeight) / 2

	// Calculate position for this cell
	posX := offsetX + float32(p.X)*cellSize
	posY := offsetY + float32(p.Y)*cellSize

	op := &ebiten.DrawImageOptions{}

	// Get the original image dimensions
	imgWidth, imgHeight := float64(img.Bounds().Dx()), float64(img.Bounds().Dy())

	// Calculate scale factors to fit the image to cell size
	scaleX := float64(cellSize) / imgWidth
	scaleY := float64(cellSize) / imgHeight

	// normalize rotations
	rotations = rotations % 4
	if rotations < 0 {
		rotations += 4
	}

	angleInRadians := float64(rotations) * math.Pi / 2

	halfWidth, halfHeight := imgWidth/2, imgHeight/2
	op.GeoM.Translate(-halfWidth, -halfHeight)

	// Apply rotation (90 degrees * number of rotations)
	op.GeoM.Rotate(angleInRadians)

	// Move back from center
	op.GeoM.Translate(halfWidth, halfHeight)

	// Apply scaling
	op.GeoM.Scale(scaleX, scaleY)

	// Apply translation (position)
	op.GeoM.Translate(float64(posX), float64(posY))

	screen.DrawImage(img, op)
}

// LerpColor linearly interpolates between two colors
// t is the interpolation factor (0.0 = color1, 1.0 = color2)
func lerpColor(color1, color2 color.Color, t float32) color.Color {
	// Clamp t between 0 and 1
	t = float32(math.Max(0, math.Min(1, float64(t))))

	// Extract RGBA components (0-255 range with alpha)
	r1, g1, b1, a1 := color1.RGBA()
	r2, g2, b2, a2 := color2.RGBA()

	// The color.RGBA() method returns values in range 0-65535,
	// so we need to convert back to 0-255 range
	r1, g1, b1, a1 = r1>>8, g1>>8, b1>>8, a1>>8
	r2, g2, b2, a2 = r2>>8, g2>>8, b2>>8, a2>>8

	// Linear interpolation for each component
	r := uint8(float32(r1)*t + float32(r2)*(1-t))
	g := uint8(float32(g1)*t + float32(g2)*(1-t))
	b := uint8(float32(b1)*t + float32(b2)*(1-t))
	a := uint8(float32(a1)*t + float32(a2)*(1-t))

	return color.RGBA{r, g, b, a}
}

const (
	segmentStraight = iota
	segmentBentLeft
	segmentBentRight
)

func getBodySegmentConfig(prevDir, nextDir Direction) (segmentType, rotations int) {
	switch prevDir {
	case DirNorth:
		switch nextDir {
		case DirNorth:
			return segmentStraight, 1
		case DirSouth:
			panic("invalid configuration")
		case DirEast:
			return segmentBentRight, 1
		case DirWest:
			return segmentBentLeft, 0
		}

	case DirSouth:
		switch nextDir {
		case DirNorth:
			panic("invalid configuration")
		case DirSouth:
			return segmentStraight, 3
		case DirEast:
			return segmentBentLeft, 2
		case DirWest:
			return segmentBentRight, 3
		}

	case DirEast:
		switch nextDir {
		case DirNorth:
			return segmentBentLeft, 1
		case DirSouth:
			return segmentBentRight, 2
		case DirEast:
			return segmentStraight, 2
		case DirWest:
			panic("invalid configuration")
		}

	case DirWest:
		switch nextDir {
		case DirNorth:
			return segmentBentRight, 0
		case DirSouth:
			return segmentBentLeft, 3
		case DirEast:
			panic("invalid configuration")
		case DirWest:
			return segmentStraight, 0
		}
	}
	panic("invalid directions")
}

func (g *Game) drawSnakeWithSkin(screen *ebiten.Image, atlas *ebiten.Image, snake Snake) {
	// the size of the texture
	aw, ah := atlas.Bounds().Dx(), atlas.Bounds().Dy()
	// the size of each cell in the atlas
	cw, ch := aw/3, ah/2
	// the 4 components
	head := atlas.SubImage(image.Rect(0, 0, cw, ch)).(*ebiten.Image) // head facing left
	segm := []*ebiten.Image{
		atlas.SubImage(image.Rect(cw, 0, 2*cw, ch)).(*ebiten.Image),    // straight body (═)
		atlas.SubImage(image.Rect(2*cw, 0, 3*cw, ch)).(*ebiten.Image),  // bent left body (╗)
		atlas.SubImage(image.Rect(cw, ch, 2*cw, 2*ch)).(*ebiten.Image), // bent right body (╔)
	}
	tail := atlas.SubImage(image.Rect(2*cw, ch, 3*cw, 2*ch)).(*ebiten.Image) // tail (facing up towards body)

	// iterate over snake.Body() and draw the image at the appropriate grid position
	if snake.IsDead() {
		return
	}

	body := snake.Body()
	for i, p := range body {
		var img *ebiten.Image
		var rotations int
		if i == 0 {
			img = head
			rotations = int(snake.Direction())
		} else if i == len(body)-1 {
			img = tail
			d := body[i].DirectionTo(body[i-1])
			rotations = int(d) - 1
		} else {
			a, b, c := body[i-1], body[i], body[i+1]
			prevDir := c.DirectionTo(b)
			nextDir := b.DirectionTo(a)
			t, r := getBodySegmentConfig(prevDir, nextDir)
			rotations = r
			img = segm[t]
		}

		g.drawGridCellImage(screen, img, p, rotations)
	}
}

func (g *Game) Render(screen *ebiten.Image) {
	// Background color
	screen.Fill(color.RGBA{0x06, 0x06, 0x06, 0xFF})
	// Draw the grid background
	for y := range g.config.Height {
		for x := range g.config.Width {
			g.drawGridCell(screen, GridPoint{x, y}, 0.9, color.RGBA{0x0D, 0x0D, 0x0D, 0xFF})
		}
	}

	// Draw the food
	for p, v := range g.state.food {
		fresh := color.RGBA{0x00, 0xFF, 0x00, 0xFF}  // green
		rotted := color.RGBA{0xFF, 0x00, 0x00, 0xFF} // red

		f := float32(1.0)
		if g.config.FoodLifetime > 0 {
			f = float32(v) / float32(g.config.FoodLifetime)
		}

		scale := 0.6*f + 0.3*(1.-f)
		actual := lerpColor(fresh, rotted, f)
		g.drawGridCell(screen, p, scale, actual)
	}

	// Draw the snakes
	for id, s := range g.state.snakes {
		g.drawSnakeWithSkin(screen, g.getSkin(id), *s)
	}

	snakes := make([]*Snake, len(g.players))
	for id := range len(snakes) {
		snakes[id] = g.state.snakes[id]
	}
	// sort.Sort(ByScore(snakes))
	slices.SortFunc(snakes, func(a, b *Snake) int {
		// Compare by MaxLength
		if a.MaxLength != b.MaxLength {
			return b.MaxLength - a.MaxLength
		}
		// Then by CurrentLength
		if a.CurrentLength != b.CurrentLength {
			return b.CurrentLength - a.CurrentLength
		}
		// Then by Kills
		if a.Kills != b.Kills {
			return b.Kills - a.Kills
		}
		// Finally by Deaths (fewer is better)
		if a.Deaths != b.Deaths {
			return (a.Deaths + a.Suicides) - (b.Deaths + b.Suicides)
		}
		// Equal
		return 0
	})

	displayStrings := make([]string, len(g.players))
	for i, s := range snakes {
		displayStrings[i] = fmt.Sprintf("%d. %s: %d (max: %d) K: %d D: %d", i, s.player.Name(), s.CurrentLength, s.MaxLength, s.Kills, s.Deaths+s.Suicides)
	}
	ebitenutil.DebugPrint(screen, strings.Join(displayStrings, "\n"))
}

type renderState struct {
	skins map[int]*ebiten.Image
}

func (g *Game) getSkin(id int) *ebiten.Image {
	p := g.players[id]
	if g.renderState.skins == nil {
		g.renderState.skins = make(map[int]*ebiten.Image)
	}

	skin, cached := g.renderState.skins[id]
	if cached {
		return skin
	}

	atlas, _, _ := image.Decode(bytes.NewReader(images.DefaultAtlas))
	palAtlas := atlas.(*image.Paletted)
	palAtlas.Palette = buildPalette(p.Color())
	skin = ebiten.NewImageFromImage(atlas)
	g.renderState.skins[id] = skin
	return skin
}

func buildPalette(c color.Color) color.Palette {
	transparent := color.RGBA{0x00, 0x00, 0x00, 0x00}
	dark := gamut.Darker(c, 0.3)
	complement := gamut.Complementary(c)

	return color.Palette{
		transparent,
		c, // primary color
		complement,
		dark,
	}
}
