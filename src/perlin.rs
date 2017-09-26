const p: [usize; 512] = 
	[183, 248, 164,  81, 145, 214, 152,  67, 104,  21, 240, 165, 110,
	23,  33,  79, 108, 209, 228, 129, 101, 222, 238,  29, 195, 141,
	113, 189, 200,  19, 184, 130,  36, 112, 168,   4,  30,  44, 215,
	65, 211, 199, 251, 175, 137,  40, 128, 135, 242,  63, 244,  55,
	12, 232, 245,  32,  62, 117, 105,   1, 171,  83, 138,  51,  95,
	98, 212,  60, 182,  41, 132,  38,  54,  74,  88, 250, 143, 216,
	196, 234,  73, 127,  15,  76, 197, 157,  27,  71, 192,  26, 239,
	43, 114, 136,  61,   0, 206, 186, 253,  34, 119, 151,  87, 187,
	46,  86, 100, 226,  13,  56, 249, 155, 180, 231,  72, 161, 102,
	201, 122, 150, 173,   2, 144, 179,  16,  25, 118, 194, 166, 227,
	109,   3, 185,  84,  69, 236,  90,  59, 111, 176,  94, 225,  58,
	28,  82, 159, 154,   5, 213,   8,  78, 134,  48, 160, 229, 146,
	50, 202, 106, 126,  57,  49, 220, 170, 148,   6, 188,  70, 190,
	99,  10, 172, 208,  18, 123, 147,  91, 235, 233, 247,  45,  66,
	223, 255,  47, 177,  93, 153,  89, 131, 219, 218, 149, 142, 243,
	230, 246,  92,  68,  85, 103, 181, 254,  11, 163, 237, 107,  64,
	193, 191, 174, 241,  52,  35, 162,  39, 203, 205,  80, 158,   9,
	75,  17, 116, 169,  24,  22, 125,  96, 204, 178,  97, 224,  37,
	42, 120, 115, 207, 217,  31, 210, 124, 252, 140, 221, 133, 156,
	198, 139,  14,   7,  77, 121, 167,  20,  53, 183, 248, 164,  81,
	145, 214, 152,  67, 104,  21, 240, 165, 110,  23,  33,  79, 108,
	209, 228, 129, 101, 222, 238,  29, 195, 141, 113, 189, 200,  19,
	184, 130,  36, 112, 168,   4,  30,  44, 215,  65, 211, 199, 251,
	175, 137,  40, 128, 135, 242,  63, 244,  55,  12, 232, 245,  32,
	62, 117, 105,   1, 171,  83, 138,  51,  95,  98, 212,  60, 182,
	41, 132,  38,  54,  74,  88, 250, 143, 216, 196, 234,  73, 127,
	15,  76, 197, 157,  27,  71, 192,  26, 239,  43, 114, 136,  61,
	 0, 206, 186, 253,  34, 119, 151,  87, 187,  46,  86, 100, 226,
	13,  56, 249, 155, 180, 231,  72, 161, 102, 201, 122, 150, 173,
	 2, 144, 179,  16,  25, 118, 194, 166, 227, 109,   3, 185,  84,
	69, 236,  90,  59, 111, 176,  94, 225,  58,  28,  82, 159, 154,
	 5, 213,   8,  78, 134,  48, 160, 229, 146,  50, 202, 106, 126,
	57,  49, 220, 170, 148,   6, 188,  70, 190,  99,  10, 172, 208,
	18, 123, 147,  91, 235, 233, 247,  45,  66, 223, 255,  47, 177,
	93, 153,  89, 131, 219, 218, 149, 142, 243, 230, 246,  92,  68,
	85, 103, 181, 254,  11, 163, 237, 107,  64, 193, 191, 174, 241,
	52,  35, 162,  39, 203, 205,  80, 158,   9,  75,  17, 116, 169,
	24,  22, 125,  96, 204, 178,  97, 224,  37,  42, 120, 115, 207,
	217,  31, 210, 124, 252, 140, 221, 133, 156, 198, 139,  14,   7,
	77, 121, 167,  20,  53];

fn fade(t: f64) -> f64 {
    t * t * t * (t * (6. * t - 15.)+ 10.)
}

fn grad(h: usize, x: f64, y: f64) -> f64 {
	if h < 0 || h > 3 { panic!("invalid hash"); }
	match h {
		0 => x + y,
		1 => x - y,
		2 => -x + y,
		_ => -x - y,
	}
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}

pub fn perlin(x: f64, y: f64) -> f64 {

    let xi = (x.floor() as usize) % 255;
    let yi = (y.floor() as usize) % 255;
    
    let xf = x - xi as f64;
    let yf = y - yi as f64;
    
    let u = fade(xf);
    let v = fade(yf);

    let aa = p[p[xi] + yi] % 4;
    let ab = p[p[xi] + yi + 1] % 4;
    let ba = p[p[xi+1] + yi] % 4;
    let bb = p[p[xi+1] + yi + 1] % 4;

    let g1 = grad(aa, xf, yf);
    let g2 = grad(ab, xf, yf-1.);
    let g3 = grad(ba, xf-1., yf);
    let g4 = grad(bb, xf-1., yf-1.);
    
    let x1 = lerp(g1, g3, u);
    let x2 = lerp(g2, g4, u);
    
    return lerp(x1, x2, v);
}