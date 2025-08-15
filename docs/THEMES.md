# Theme System 🎨

Rusty2048 features a beautiful theme system with 5 different color schemes to enhance your gaming experience.

## Available Themes

### 1. Classic 🌟
The original 2048 color scheme with warm, earthy tones.
- **Background**: Light beige (#faf8ef)
- **Grid**: Warm gray (#bbada0)
- **Tiles**: Traditional 2048 colors with browns and oranges
- **Perfect for**: Traditional 2048 experience

### 2. Dark 🌙
A sleek dark theme for modern terminals and night gaming.
- **Background**: Dark gray (#1a1a1a)
- **Grid**: Medium gray (#2d2d2d)
- **Tiles**: Grayscale progression with bright accents
- **Perfect for**: Modern terminals, low-light environments

### 3. Neon ⚡
Vibrant neon colors for a cyberpunk aesthetic.
- **Background**: Pure black (#000000)
- **Grid**: Dark purple (#1a0033)
- **Tiles**: Bright neon colors (pink, cyan, yellow, etc.)
- **Perfect for**: High contrast displays, retro gaming feel

### 4. Retro 🕹️
Classic terminal green theme reminiscent of old computers.
- **Background**: Dark gray (#2b2b2b)
- **Grid**: Medium gray (#404040)
- **Tiles**: Various shades of green
- **Perfect for**: Nostalgic terminal experience

### 5. Pastel 🌸
Soft, gentle pastel colors for a calming experience.
- **Background**: Light gray (#f8f9fa)
- **Grid**: Light gray (#e9ecef)
- **Tiles**: Soft pastel colors (pink, blue, yellow, etc.)
- **Perfect for**: Relaxed gaming, easy on the eyes

## How to Use

### Quick Theme Switching
- **T**: Cycle through all themes
- **1**: Classic theme
- **2**: Dark theme
- **3**: Neon theme
- **4**: Retro theme
- **5**: Pastel theme

### Theme Help
- **H**: Toggle theme help display
- Shows available themes and keyboard shortcuts

## Theme Features

### Automatic Text Color
Each theme automatically calculates the best text color (black or white) based on tile background brightness for optimal readability.

### Consistent UI Colors
All UI elements (title, score, moves, time) use theme-appropriate colors that complement the overall design.

### Smooth Transitions
Themes switch instantly without any flickering or visual artifacts.

## Customization

Themes are defined in the `shared` module and can be easily extended:

1. Add new theme definitions in `shared/src/lib.rs`
2. Update the `all_themes()` function
3. Add keyboard shortcuts in `cli/src/main.rs`

## Technical Details

- **Color Format**: Hex colors (#RRGGBB)
- **Color Conversion**: Automatic hex to RGB conversion for ratatui
- **Luminance Calculation**: Automatic text color selection based on background brightness
- **Memory Efficient**: Themes are shared across the workspace

## Future Enhancements

- [ ] Custom theme creation
- [ ] Theme persistence across sessions
- [ ] Animated theme transitions
- [ ] Seasonal themes
- [ ] User-uploaded themes

---

*Enjoy the beautiful colors while playing 2048!* 🎮✨
