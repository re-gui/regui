use std::str::FromStr;
use enum_iterator::Sequence;

macro_rules! color_box {
    // see https://stackoverflow.com/questions/35361986/css-gradient-checkerboard-pattern
    ($string_hex_value:literal) => {
        concat!(
            r##"<span style="background:url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAIAAAD91JpzAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAARSURBVBhXYwCC////gzEDAwAp5AX7bk/yfwAAAABJRU5ErkJggg==');background-size: 10px 10px;width: 4em;height: 2em;display:block;image-rendering: pixelated;border:1px solid gray;"><span style="display:block;height:100%;background:"##,
            "#", $string_hex_value,
            r##""></span></span>"##
        )
    }
}

macro_rules! input_color {
    ($string_hex_no_h_value:literal) => {
        concat!(
            r##"<input type="color" value="#"##,
            $string_hex_no_h_value,
            r##"">"##
        )
    }
}

macro_rules! geenrate_table {
    ($($name:ident $string_hex_value:literal $string_hex_no_h_value:literal)*,) => {
        concat!(
            "|  Name | visual | Hex |\n",
            "| ----: | ------ | --- |\n",
            $(concat!("| [`", stringify!($name), "`](CssNamedColors::", stringify!($name), ") | ", color_box!($string_hex_value), " |`#", $string_hex_value, "` |\n")),*
        )
    }
}

macro_rules! make_colors {
    (
        $(
            $name:ident $hex_value:literal $string_hex_value:literal $string_hex_no_h_value:literal
            rgba($r:literal, $g:literal, $b:literal, $a:literal)
            hsla($h:literal, $s:literal, $v:literal, $a_hsl:literal)
            ; $(#[$doc:meta])*
        )*
    ) => {
        /// CSS named colors.
        ///
        /// See <https://developer.mozilla.org/en-US/docs/Web/CSS/named-color> for more information.
        ///
        #[doc = geenrate_table!(
            $($name $string_hex_value $string_hex_no_h_value)*,
        )]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Sequence)]
        #[allow(non_camel_case_types)]
        pub enum CssNamedColors {
            $(
                $(#[$doc])*
                #[doc = concat!(
                    "`#", $string_hex_value, "` ",
                    r##"<span style="color:#"##, $string_hex_value, r##"">◼</span>"##,
                    //r##"<span style="display:inline-block;background:#"##,
                    //$string_hex_value,
                    //r##";width:1em;height:1em;box-sizing:border-box;border:1px solid black;"></span>"##,
                    "\n\n",
                    input_color!($string_hex_no_h_value), "\n",
                    "\n\n",
                    "# Representations\n",
                    "- `#", $string_hex_value, "`\n",
                    "- `rgba(", $r, ", ", $g, ", ", $b, ", ", $a, ")`\n",
                    "- `rgb(", $r, ", ", $g, ", ", $b, ")`\n",
                    "- `hsla(", $h, ", ", $s, ", ", $v, ", ", $a_hsl, ")`\n",
                    "- `hsl(", $h, ", ", $s, ", ", $v, ")`\n",
                )]
                $name,
            )*
        }
        impl CssNamedColors {
            /// Returns the CSS hex value of the color in the form `0xRRGGBBAA`.
            ///
            /// <https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color>
            pub const fn to_css_hex(&self) -> u32 {
                match self {
                    $(
                        CssNamedColors::$name => $hex_value,
                    )*
                }
            }
        }
        impl FromStr for CssNamedColors { // TODO impl const
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($name) => Ok(CssNamedColors::$name),
                    )*
                    _ => Err(()),
                }
            }
        }
    };
}

impl CssNamedColors {
    pub fn r(&self) -> u8 {
        let hex = self.to_css_hex();
        ((hex >> 24) & 0xff) as u8
    }
    pub fn g(&self) -> u8 {
        let hex = self.to_css_hex();
        ((hex >> 16) & 0xff) as u8
    }
    pub fn b(&self) -> u8 {
        let hex = self.to_css_hex();
        ((hex >> 8) & 0xff) as u8
    }
    pub fn a(&self) -> u8 {
        let hex = self.to_css_hex();
        (hex & 0xff) as u8
    }
    pub fn rgb(&self) -> [u8; 3] {
        [self.r(), self.g(), self.b()]
    }
    pub fn rgba(&self) -> [u8; 4] {
        [self.r(), self.g(), self.b(), self.a()]
    }
    pub fn hsl(&self) -> [f32; 3] {
        let hsl = hsl::HSL::from_rgb(&self.rgb());
        [hsl.h as f32, hsl.s as f32, hsl.l as f32]
    }
    pub fn hsla(&self) -> [f32; 4] {
        let hsl = hsl::HSL::from_rgb(&self.rgb());
        [hsl.h as f32, hsl.s as f32, hsl.l as f32, self.a() as f32 / 255.0]
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/named-color
make_colors!(
    // standard colors
    black                0x000000ff "000000ff" "000000" rgba(  0,   0,   0, 255) hsla(     0,     0,     0, 1); /// CIAO
    silver               0xc0c0c0ff "c0c0c0ff" "c0c0c0" rgba(192, 192, 192, 255) hsla(     0,     0, 0.753, 1);
    gray                 0x808080ff "808080ff" "808080" rgba(128, 128, 128, 255) hsla(     0,     0, 0.502, 1);
    white                0xffffffff "ffffffff" "ffffff" rgba(255, 255, 255, 255) hsla(     0,     0,     1, 1);
    maroon               0x800000ff "800000ff" "800000" rgba(128,   0,   0, 255) hsla(     0,     1, 0.251, 1);
    red                  0xff0000ff "ff0000ff" "ff0000" rgba(255,   0,   0, 255) hsla(     0,     1,   0.5, 1);
    purple               0x800080ff "800080ff" "800080" rgba(128,   0, 128, 255) hsla(   300,     1, 0.251, 1);
    fuchsia              0xff00ffff "ff00ffff" "ff00ff" rgba(255,   0, 255, 255) hsla(   300,     1,   0.5, 1);
    green                0x008000ff "008000ff" "008000" rgba(  0, 128,   0, 255) hsla(   120,     1, 0.251, 1);
    lime                 0x00ff00ff "00ff00ff" "00ff00" rgba(  0, 255,   0, 255) hsla(   120,     1,   0.5, 1);
    olive                0x808000ff "808000ff" "808000" rgba(128, 128,   0, 255) hsla(    60,     1, 0.251, 1);
    yellow               0xffff00ff "ffff00ff" "ffff00" rgba(255, 255,   0, 255) hsla(    60,     1,   0.5, 1);
    navy                 0x000080ff "000080ff" "000080" rgba(  0,   0, 128, 255) hsla(   240,     1, 0.251, 1);
    blue                 0x0000ffff "0000ffff" "0000ff" rgba(  0,   0, 255, 255) hsla(   240,     1,   0.5, 1);
    teal                 0x008080ff "008080ff" "008080" rgba(  0, 128, 128, 255) hsla(   180,     1, 0.251, 1);
    aqua                 0x00ffffff "00ffffff" "00ffff" rgba(  0, 255, 255, 255) hsla(   180,     1,   0.5, 1);

    aliceblue            0xf0f8ffff "f0f8ffff" "f0f8ff" rgba(240, 248, 255, 255) hsla(   208,     1, 0.971, 1);
    antiquewhite         0xfaebd7ff "faebd7ff" "faebd7" rgba(250, 235, 215, 255) hsla( 34.29, 0.778, 0.912, 1);
    //aqua               0x00ffffff "00ffffff" "00ffff";
    aquamarine           0x7fffd4ff "7fffd4ff" "7fffd4" rgba(127, 255, 212, 255) hsla(159.84,     1, 0.749, 1);
    azure                0xf0ffffff "f0ffffff" "f0ffff" rgba(240, 255, 255, 255) hsla(   180,     1, 0.971, 1);
    beige                0xf5f5dcff "f5f5dcff" "f5f5dc" rgba(245, 245, 220, 255) hsla(    60, 0.556, 0.912, 1);
    bisque               0xffe4c4ff "ffe4c4ff" "ffe4c4" rgba(255, 228, 196, 255) hsla( 32.54,     1, 0.884, 1);
    //black              0x000000ff "000000ff" "000000";
    blanchedalmond       0xffebcdff "ffebcdff" "ffebcd" rgba(255, 235, 205, 255) hsla(    36,     1, 0.902, 1);
    //blue               0x0000ffff "0000ffff" "0000ff";
    blueviolet           0x8a2be2ff "8a2be2ff" "8a2be2" rgba(138,  43, 226, 255) hsla(271.15, 0.759, 0.527, 1);
    brown                0xa52a2aff "a52a2aff" "a52a2a" rgba(165,  42,  42, 255) hsla(     0, 0.594, 0.406, 1);
    burlywood            0xdeb887ff "deb887ff" "deb887" rgba(222, 184, 135, 255) hsla( 33.79, 0.569,   0.7, 1);
    cadetblue            0x5f9ea0ff "5f9ea0ff" "5f9ea0" rgba( 95, 158, 160, 255) hsla(181.85, 0.255,   0.5, 1);
    chartreuse           0x7fff00ff "7fff00ff" "7fff00" rgba(127, 255,   0, 255) hsla( 90.12,     1,   0.5, 1);
    chocolate            0xd2691eff "d2691eff" "d2691e" rgba(210, 105,  30, 255) hsla(    25,  0.75, 0.471, 1);
    coral                0xff7f50ff "ff7f50ff" "ff7f50" rgba(255, 127,  80, 255) hsla( 16.11,     1, 0.657, 1);
    cornflowerblue       0x6495edff "6495edff" "6495ed" rgba(100, 149, 237, 255) hsla(218.54, 0.792, 0.661, 1);
    cornsilk             0xfff8dcff "fff8dcff" "fff8dc" rgba(255, 248, 220, 255) hsla(    48,     1, 0.931, 1);
    crimson              0xdc143cff "dc143cff" "dc143c" rgba(220,  20,  60, 255) hsla(   348, 0.833, 0.471, 1);
    cyan                 0x00ffffff "00ffffff" "00ffff" rgba(  0, 255, 255, 255) hsla(   180,     1,   0.5, 1); /// synonym of aqua
    darkblue             0x00008bff "00008bff" "00008b" rgba(  0,   0, 139, 255) hsla(   240,     1, 0.273, 1);
    darkcyan             0x008b8bff "008b8bff" "008b8b" rgba(  0, 139, 139, 255) hsla(   180,     1, 0.273, 1);
    darkgoldenrod        0xb8860bff "b8860bff" "b8860b" rgba(184, 134,  11, 255) hsla( 42.66, 0.887, 0.382, 1);
    darkgray             0xa9a9a9ff "a9a9a9ff" "a9a9a9" rgba(169, 169, 169, 255) hsla(     0,     0, 0.663, 1);
    darkgreen            0x006400ff "006400ff" "006400" rgba(  0, 100,   0, 255) hsla(   120,     1, 0.196, 1);
    darkgrey             0xa9a9a9ff "a9a9a9ff" "a9a9a9" rgba(169, 169, 169, 255) hsla(     0,     0, 0.663, 1);
    darkkhaki            0xbdb76bff "bdb76bff" "bdb76b" rgba(189, 183, 107, 255) hsla( 55.61, 0.383,  0.58, 1);
    darkmagenta          0x8b008bff "8b008bff" "8b008b" rgba(139,   0, 139, 255) hsla(   300,     1, 0.273, 1);
    darkolivegreen       0x556b2fff "556b2fff" "556b2f" rgba( 85, 107,  47, 255) hsla(    82,  0.39, 0.302, 1);
    darkorange           0xff8c00ff "ff8c00ff" "ff8c00" rgba(255, 140,   0, 255) hsla( 32.94,     1,   0.5, 1);
    darkorchid           0x9932ccff "9932ccff" "9932cc" rgba(153,  50, 204, 255) hsla(280.13, 0.606, 0.498, 1);
    darkred              0x8b0000ff "8b0000ff" "8b0000" rgba(139,   0,   0, 255) hsla(     0,     1, 0.273, 1);
    darksalmon           0xe9967aff "e9967aff" "e9967a" rgba(233, 150, 122, 255) hsla( 15.14, 0.716, 0.696, 1);
    darkseagreen         0x8fbc8fff "8fbc8fff" "8fbc8f" rgba(143, 188, 143, 255) hsla(   120, 0.251, 0.649, 1);
    darkslateblue        0x483d8bff "483d8bff" "483d8b" rgba( 72,  61, 139, 255) hsla(248.46,  0.39, 0.392, 1);
    darkslategray        0x2f4f4fff "2f4f4fff" "2f4f4f" rgba( 47,  79,  79, 255) hsla(   180, 0.254, 0.247, 1);
    darkslategrey        0x2f4f4fff "2f4f4fff" "2f4f4f" rgba( 47,  79,  79, 255) hsla(   180, 0.254, 0.247, 1);
    darkturquoise        0x00ced1ff "00ced1ff" "00ced1" rgba(  0, 206, 209, 255) hsla(180.86,     1,  0.41, 1);
    darkviolet           0x9400d3ff "9400d3ff" "9400d3" rgba(148,   0, 211, 255) hsla(282.09,     1, 0.414, 1);
    deeppink             0xff1493ff "ff1493ff" "ff1493" rgba(255,  20, 147, 255) hsla(327.57,     1, 0.539, 1);
    deepskyblue          0x00bfffff "00bfffff" "00bfff" rgba(  0, 191, 255, 255) hsla(195.06,     1,   0.5, 1);
    dimgray              0x696969ff "696969ff" "696969" rgba(105, 105, 105, 255) hsla(     0,     0, 0.412, 1);
    dimgrey              0x696969ff "696969ff" "696969" rgba(105, 105, 105, 255) hsla(     0,     0, 0.412, 1);
    dodgerblue           0x1e90ffff "1e90ffff" "1e90ff" rgba( 30, 144, 255, 255) hsla( 209.6,     1, 0.559, 1);
    firebrick            0xb22222ff "b22222ff" "b22222" rgba(178,  34,  34, 255) hsla(     0, 0.679, 0.416, 1);
    floralwhite          0xfffaf0ff "fffaf0ff" "fffaf0" rgba(255, 250, 240, 255) hsla(    40,     1, 0.971, 1);
    forestgreen          0x228b22ff "228b22ff" "228b22" rgba( 34, 139,  34, 255) hsla(   120, 0.607, 0.339, 1);
    //fuchsia            0xff00ffff "ff00ffff" "ff00ff" "ff00ff";
    gainsboro            0xdcdcdcff "dcdcdcff" "dcdcdc" rgba(220, 220, 220, 255) hsla(     0,     0, 0.863, 1);
    ghostwhite           0xf8f8ffff "f8f8ffff" "f8f8ff" rgba(248, 248, 255, 255) hsla(   240,     1, 0.986, 1);
    gold                 0xffd700ff "ffd700ff" "ffd700" rgba(255, 215,   0, 255) hsla( 50.59,     1,   0.5, 1);
    goldenrod            0xdaa520ff "daa520ff" "daa520" rgba(218, 165,  32, 255) hsla(  42.9, 0.744,  0.49, 1);
    //gray               0x808080ff "808080ff" "808080";
    //green              0x008000ff "008000ff" "008000";
    greenyellow          0xadff2fff "adff2fff" "adff2f" rgba(173, 255,  47, 255) hsla( 83.65,     1, 0.592, 1);
    grey                 0x808080ff "808080ff" "808080" rgba(128, 128, 128, 255) hsla(     0,     0, 0.502, 1); /// synonym of gray
    honeydew             0xf0fff0ff "f0fff0ff" "f0fff0" rgba(240, 255, 240, 255) hsla(   120,     1, 0.971, 1);
    hotpink              0xff69b4ff "ff69b4ff" "ff69b4" rgba(255, 105, 180, 255) hsla(   330,     1, 0.706, 1);
    indianred            0xcd5c5cff "cd5c5cff" "cd5c5c" rgba(205,  92,  92, 255) hsla(     0, 0.531, 0.582, 1);
    indigo               0x4b0082ff "4b0082ff" "4b0082" rgba( 75,   0, 130, 255) hsla(274.62,     1, 0.255, 1);
    ivory                0xfffff0ff "fffff0ff" "fffff0" rgba(255, 255, 240, 255) hsla(    60,     1, 0.971, 1);
    khaki                0xf0e68cff "f0e68cff" "f0e68c" rgba(240, 230, 140, 255) hsla(    54, 0.769, 0.745, 1);
    lavender             0xe6e6faff "e6e6faff" "e6e6fa" rgba(230, 230, 250, 255) hsla(   240, 0.667, 0.941, 1);
    lavenderblush        0xfff0f5ff "fff0f5ff" "fff0f5" rgba(255, 240, 245, 255) hsla(   340,     1, 0.971, 1);
    lawngreen            0x7cfc00ff "7cfc00ff" "7cfc00" rgba(124, 252,   0, 255) hsla( 90.48,     1, 0.494, 1);
    lemonchiffon         0xfffacdff "fffacdff" "fffacd" rgba(255, 250, 205, 255) hsla(    54,     1, 0.902, 1);
    lightblue            0xadd8e6ff "add8e6ff" "add8e6" rgba(173, 216, 230, 255) hsla(194.74, 0.533,  0.79, 1);
    lightcoral           0xf08080ff "f08080ff" "f08080" rgba(240, 128, 128, 255) hsla(     0, 0.789, 0.722, 1);
    lightcyan            0xe0ffffff "e0ffffff" "e0ffff" rgba(224, 255, 255, 255) hsla(   180,     1, 0.939, 1);
    lightgoldenrodyellow 0xfafad2ff "fafad2ff" "fafad2" rgba(250, 250, 210, 255) hsla(    60,   0.8, 0.902, 1);
    lightgray            0xd3d3d3ff "d3d3d3ff" "d3d3d3" rgba(211, 211, 211, 255) hsla(     0,     0, 0.827, 1);
    lightgreen           0x90ee90ff "90ee90ff" "90ee90" rgba(144, 238, 144, 255) hsla(   120, 0.734, 0.749, 1);
    lightgrey            0xd3d3d3ff "d3d3d3ff" "d3d3d3" rgba(211, 211, 211, 255) hsla(     0,     0, 0.827, 1);
    lightpink            0xffb6c1ff "ffb6c1ff" "ffb6c1" rgba(255, 182, 193, 255) hsla(350.96,     1, 0.857, 1);
    lightsalmon          0xffa07aff "ffa07aff" "ffa07a" rgba(255, 160, 122, 255) hsla( 17.14,     1, 0.739, 1);
    lightseagreen        0x20b2aaff "20b2aaff" "20b2aa" rgba( 32, 178, 170, 255) hsla(176.71, 0.695, 0.412, 1);
    lightskyblue         0x87cefaff "87cefaff" "87cefa" rgba(135, 206, 250, 255) hsla(202.96,  0.92, 0.755, 1);
    lightslategray       0x778899ff "778899ff" "778899" rgba(119, 136, 153, 255) hsla(   210, 0.143, 0.533, 1);
    lightslategrey       0x778899ff "778899ff" "778899" rgba(119, 136, 153, 255) hsla(   210, 0.143, 0.533, 1);
    lightsteelblue       0xb0c4deff "b0c4deff" "b0c4de" rgba(176, 196, 222, 255) hsla(213.91, 0.411,  0.78, 1);
    lightyellow          0xffffe0ff "ffffe0ff" "ffffe0" rgba(255, 255, 224, 255) hsla(    60,     1, 0.939, 1);
    //lime               0x00ff00ff "00ff00ff" "00ff00";
    limegreen            0x32cd32ff "32cd32ff" "32cd32" rgba( 50, 205,  50, 255) hsla(   120, 0.608,   0.5, 1);
    linen                0xfaf0e6ff "faf0e6ff" "faf0e6" rgba(250, 240, 230, 255) hsla(    30, 0.667, 0.941, 1);
    magenta              0xff00ffff "ff00ffff" "ff00ff" rgba(255,   0, 255, 255) hsla(   300,     1,   0.5, 1); /// synonym of fuchsia
    //maroon             0x800000ff "800000ff" "800000";
    mediumaquamarine     0x66cdaaff "66cdaaff" "66cdaa" rgba(102, 205, 170, 255) hsla(159.61, 0.507, 0.602, 1);
    mediumblue           0x0000cdff "0000cdff" "0000cd" rgba(  0,   0, 205, 255) hsla(   240,     1, 0.402, 1);
    mediumorchid         0xba55d3ff "ba55d3ff" "ba55d3" rgba(186,  85, 211, 255) hsla( 288.1, 0.589,  0.58, 1);
    mediumpurple         0x9370dbff "9370dbff" "9370db" rgba(147, 112, 219, 255) hsla(259.63, 0.598, 0.649, 1);
    mediumseagreen       0x3cb371ff "3cb371ff" "3cb371" rgba( 60, 179, 113, 255) hsla(146.72, 0.498, 0.469, 1);
    mediumslateblue      0x7b68eeff "7b68eeff" "7b68ee" rgba(123, 104, 238, 255) hsla(248.51, 0.798, 0.671, 1);
    mediumspringgreen    0x00fa9aff "00fa9aff" "00fa9a" rgba(  0, 250, 154, 255) hsla(156.96,     1,  0.49, 1);
    mediumturquoise      0x48d1ccff "48d1ccff" "48d1cc" rgba( 72, 209, 204, 255) hsla(177.81, 0.598, 0.551, 1);
    mediumvioletred      0xc71585ff "c71585ff" "c71585" rgba(199,  21, 133, 255) hsla(322.25, 0.809, 0.431, 1);
    midnightblue         0x191970ff "191970ff" "191970" rgba( 25,  25, 112, 255) hsla(   240, 0.635, 0.269, 1);
    mintcream            0xf5fffaff "f5fffaff" "f5fffa" rgba(245, 255, 250, 255) hsla(   150,     1,  0.98, 1);
    mistyrose            0xffe4e1ff "ffe4e1ff" "ffe4e1" rgba(255, 228, 225, 255) hsla(     6,     1, 0.941, 1);
    moccasin             0xffe4b5ff "ffe4b5ff" "ffe4b5" rgba(255, 228, 181, 255) hsla( 38.11,     1, 0.855, 1);
    navajowhite          0xffdeadff "ffdeadff" "ffdead" rgba(255, 222, 173, 255) hsla( 35.85,     1, 0.839, 1);
    //navy               0x000080ff "000080ff" "000080";
    oldlace              0xfdf5e6ff "fdf5e6ff" "fdf5e6" rgba(253, 245, 230, 255) hsla( 39.13, 0.852, 0.947, 1);
    //olive              0x808000ff "808000ff" "808000";
    olivedrab            0x6b8e23ff "6b8e23ff" "6b8e23" rgba(107, 142,  35, 255) hsla( 79.63, 0.605, 0.347, 1);
    orange               0xffa500ff "ffa500ff" "ffa500" rgba(255, 165,   0, 255) hsla( 38.82,     1,   0.5, 1);
    orangered            0xff4500ff "ff4500ff" "ff4500" rgba(255,  69,   0, 255) hsla( 16.24,     1,   0.5, 1);
    orchid               0xda70d6ff "da70d6ff" "da70d6" rgba(218, 112, 214, 255) hsla(302.26, 0.589, 0.647, 1);
    palegoldenrod        0xeee8aaff "eee8aaff" "eee8aa" rgba(238, 232, 170, 255) hsla( 54.71, 0.667,   0.8, 1);
    palegreen            0x98fb98ff "98fb98ff" "98fb98" rgba(152, 251, 152, 255) hsla(   120, 0.925,  0.79, 1);
    paleturquoise        0xafeeeeff "afeeeeff" "afeeee" rgba(175, 238, 238, 255) hsla(   180, 0.649,  0.81, 1);
    palevioletred        0xdb7093ff "db7093ff" "db7093" rgba(219, 112, 147, 255) hsla(340.37, 0.598, 0.649, 1);
    papayawhip           0xffefd5ff "ffefd5ff" "ffefd5" rgba(255, 239, 213, 255) hsla( 37.14,     1, 0.918, 1);
    peachpuff            0xffdab9ff "ffdab9ff" "ffdab9" rgba(255, 218, 185, 255) hsla( 28.29,     1, 0.863, 1);
    peru                 0xcd853fff "cd853fff" "cd853f" rgba(205, 133,  63, 255) hsla( 29.58, 0.587, 0.525, 1);
    pink                 0xffc0cbff "ffc0cbff" "ffc0cb" rgba(255, 192, 203, 255) hsla(349.52,     1, 0.876, 1);
    plum                 0xdda0ddff "dda0ddff" "dda0dd" rgba(221, 160, 221, 255) hsla(   300, 0.473, 0.747, 1);
    powderblue           0xb0e0e6ff "b0e0e6ff" "b0e0e6" rgba(176, 224, 230, 255) hsla(186.67, 0.519, 0.796, 1);
    //purple             0x800080ff "800080ff" "800080";
    rebeccapurple        0x663399ff "663399ff" "663399" rgba(102,  51, 153, 255) hsla(   270,   0.5,   0.4, 1);
    //red                0xff0000ff "ff0000ff" "ff0000";
    rosybrown            0xbc8f8fff "bc8f8fff" "bc8f8f" rgba(188, 143, 143, 255) hsla(     0, 0.251, 0.649, 1);
    royalblue            0x4169e1ff "4169e1ff" "4169e1" rgba( 65, 105, 225, 255) hsla(   225, 0.727, 0.569, 1);
    saddlebrown          0x8b4513ff "8b4513ff" "8b4513" rgba(139,  69,  19, 255) hsla(    25, 0.759,  0.31, 1);
    salmon               0xfa8072ff "fa8072ff" "fa8072" rgba(250, 128, 114, 255) hsla(  6.18, 0.932, 0.714, 1);
    sandybrown           0xf4a460ff "f4a460ff" "f4a460" rgba(244, 164,  96, 255) hsla( 27.57, 0.871, 0.667, 1);
    seagreen             0x2e8b57ff "2e8b57ff" "2e8b57" rgba( 46, 139,  87, 255) hsla(146.45, 0.503, 0.363, 1);
    seashell             0xfff5eeff "fff5eeff" "fff5ee" rgba(255, 245, 238, 255) hsla( 24.71,     1, 0.967, 1);
    sienna               0xa0522dff "a0522dff" "a0522d" rgba(160,  82,  45, 255) hsla(  19.3, 0.561, 0.402, 1);
    //silver             0xc0c0c0ff "c0c0c0ff" "c0c0c0";
    skyblue              0x87ceebff "87ceebff" "87ceeb" rgba(135, 206, 235, 255) hsla( 197.4, 0.714, 0.725, 1);
    slateblue            0x6a5acdff "6a5acdff" "6a5acd" rgba(106,  90, 205, 255) hsla(248.35, 0.535, 0.578, 1);
    slategray            0x708090ff "708090ff" "708090" rgba(112, 128, 144, 255) hsla(   210, 0.126, 0.502, 1);
    slategrey            0x708090ff "708090ff" "708090" rgba(112, 128, 144, 255) hsla(   210, 0.126, 0.502, 1);
    snow                 0xfffafaff "fffafaff" "fffafa" rgba(255, 250, 250, 255) hsla(     0,     1,  0.99, 1);
    springgreen          0x00ff7fff "00ff7fff" "00ff7f" rgba(  0, 255, 127, 255) hsla(149.88,     1,   0.5, 1);
    steelblue            0x4682b4ff "4682b4ff" "4682b4" rgba( 70, 130, 180, 255) hsla(207.27,  0.44,  0.49, 1);
    tan                  0xd2b48cff "d2b48cff" "d2b48c" rgba(210, 180, 140, 255) hsla( 34.29, 0.437, 0.686, 1);
    //teal               0x008080ff "008080ff" "008080";
    thistle              0xd8bfd8ff "d8bfd8ff" "d8bfd8" rgba(216, 191, 216, 255) hsla(   300, 0.243, 0.798, 1);
    tomato               0xff6347ff "ff6347ff" "ff6347" rgba(255,  99,  71, 255) hsla(  9.13,     1, 0.639, 1);
    transparent          0x00000000 "00000000" "000000" rgba(  0,   0,   0,   0) hsla(     0,     0,     0, 0);
    turquoise            0x40e0d0ff "40e0d0ff" "40e0d0" rgba( 64, 224, 208, 255) hsla(   174, 0.721, 0.565, 1);
    violet               0xee82eeff "ee82eeff" "ee82ee" rgba(238, 130, 238, 255) hsla(   300, 0.761, 0.722, 1);
    wheat                0xf5deb3ff "f5deb3ff" "f5deb3" rgba(245, 222, 179, 255) hsla( 39.09, 0.767, 0.831, 1);
    //white              0xffffffff "ffffffff" "ffffff";
    whitesmoke           0xf5f5f5ff "f5f5f5ff" "f5f5f5" rgba(245, 245, 245, 255) hsla(     0,     0, 0.961, 1);
    //yellow             0xffff00ff "ffff00ff" "ffff00";
    yellowgreen          0x9acd32ff "9acd32ff" "9acd32" rgba(154, 205,  50, 255) hsla( 79.74, 0.608,   0.5, 1);
);

/*
to generate rgb and hsl:
fn main() {
    let t: Taffy;

    let it = enum_iterator::all::<CssNamedColors>();
    let file = it.take(50000000).map(|i| {
        let rgb = i.rgb();
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];
        let a = i.a();
        let hsl = i.hsl();
        let h = (hsl.h * 100.0).round() / 100.0;
        let s = (hsl.s * 1000.0).round() / 1000.0;
        let l = (hsl.l * 1000.0).round() / 1000.0;
        let a_hsl = a as f32 / 255.0;
        let a_hsl = (a_hsl * 100.0).round() / 100.0;
        format!("rgba({r: >3}, {g: >3}, {b: >3}, {a: >3}) hsla({h: >6}, {s: >5}, {l: >5}, {a_hsl: >1}) {:?}", i)
    }).collect::<Vec<_>>().join("\n");

    // wirte in "ciao.txt" file
    std::fs::write("ciao.txt", file).unwrap();
}
*/