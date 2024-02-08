#[derive(Debug, Default)]
pub struct Borders {
    pub top: BorderStyle,
    pub left: BorderStyle,
    pub right: BorderStyle,
    pub bottom: BorderStyle,
    pub split: BorderStyle
}

#[derive(Debug, Default)]
pub enum BorderStyle {
    Single,
    #[default]
    Double
}

#[derive(Debug)]
pub struct BorderChars {
    pub tl: char,
    pub tr: char,
    pub bl: char,
    pub br: char,

    pub top: char,
    pub left: char,
    pub right: char,
    pub bottom: char,

    pub left_intersect: char,
    pub right_intersect: char,
    pub split: char
}

impl Default for BorderChars {
    fn default() -> Self {
        Self::new(Borders::default())
    }
}

impl BorderChars {
    pub fn new(borders: Borders) -> Self {
        /*
        
                0	1	2	3	4	5	6	7	8	9	A	B	C	D	E	F
            B	░	▒	▓	│	┤	╡	╢	╖	╕	╣	║	╗	╝	╜	╛	┐
            C	└	┴	┬	├	─	┼	╞	╟	╚	╔	╩	╦	╠	═	╬	╧
            D	╨	╤	╥	╙	╘	╒	╓	╫	╪	┘	┌	█	▄	▌	▐	▀
        
         */

        let left_intersect = match (&borders.left, &borders.split) {
            (BorderStyle::Single, BorderStyle::Single) => '├',
            (BorderStyle::Single, BorderStyle::Double) => '╞',
            (BorderStyle::Double, BorderStyle::Single) => '╟',
            (BorderStyle::Double, BorderStyle::Double) => '╠',
        };

        let right_intersect = match (&borders.right, &borders.split) {
            (BorderStyle::Single, BorderStyle::Single) => '┤',
            (BorderStyle::Single, BorderStyle::Double) => '╡',
            (BorderStyle::Double, BorderStyle::Single) => '╢',
            (BorderStyle::Double, BorderStyle::Double) => '╣',
        };

        let tl = match (&borders.top, &borders.left) {
            (BorderStyle::Single, BorderStyle::Single) => '┌',
            (BorderStyle::Single, BorderStyle::Double) => '╓',
            (BorderStyle::Double, BorderStyle::Single) => '╒',
            (BorderStyle::Double, BorderStyle::Double) => '╔',
        };

        let tr = match(&borders.top, &borders.right) {
            (BorderStyle::Single, BorderStyle::Single) => '┐',
            (BorderStyle::Single, BorderStyle::Double) => '╖',
            (BorderStyle::Double, BorderStyle::Single) => '╕',
            (BorderStyle::Double, BorderStyle::Double) => '╗',
        };

        let bl = match(&borders.bottom, &borders.left) {
            (BorderStyle::Single, BorderStyle::Single) => '└',
            (BorderStyle::Single, BorderStyle::Double) => '╙',
            (BorderStyle::Double, BorderStyle::Single) => '╘',
            (BorderStyle::Double, BorderStyle::Double) => '╚',
        };

        let br = match(&borders.bottom, &borders.right) {
            (BorderStyle::Single, BorderStyle::Single) => '┘',
            (BorderStyle::Single, BorderStyle::Double) => '╜',
            (BorderStyle::Double, BorderStyle::Single) => '╛',
            (BorderStyle::Double, BorderStyle::Double) => '╝',
        };

        let top = match &borders.top {
            BorderStyle::Single => '─',
            BorderStyle::Double => '═',
        };

        let left = match &borders.left {
            BorderStyle::Single => '│',
            BorderStyle::Double => '║',
        };

        let right = match &borders.right {
            BorderStyle::Single => '│',
            BorderStyle::Double => '║',
        };

        let bottom = match &borders.bottom {
            BorderStyle::Single => '─',
            BorderStyle::Double => '═',
        };

        let split = match &borders.split {
            BorderStyle::Single => '─',
            BorderStyle::Double => '═',
        };

        Self { tl, tr, bl, br, top, left, right, bottom, left_intersect, right_intersect, split }

    }
}