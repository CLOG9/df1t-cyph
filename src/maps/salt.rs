const LETTER_MAP: [(&str, &[(&str, i32); 1]); 26] = [
    ("A", &[("Po", 70)]),
    ("B", &[("Rq", 80)]),
    ("C", &[("ts", 36)]),
    ("D", &[("rt", 52)]),
    ("E", &[("ww", 59)]),
    ("F", &[("zz", 32)]),
    ("G", &[("cD", 99)]),
    ("H", &[("ef", 50)]),
    ("I", &[("gh", 97)]),
    ("J", &[("kl", 81)]),
    ("K", &[("mn", 42)]),
    ("L", &[("op", 11)]),
    ("M", &[("qr", 33)]),
    ("N", &[("st", 88)]),
    ("O", &[("wx", 69)]),
    ("S", &[("yz", 94)]),
    ("T", &[("cd", 56)]),
    //
    ("P", &[("Rg", 12)]),
    ("Q", &[("Rs", 55)]),
    ("R", &[("oI", 98)]),
    //
    ("W", &[("Tu", 31)]),
    ("Z", &[("VW", 85)]),
    ("V", &[("xW", 10)]),
    ("Y", &[("Uv", 58)]),
    ("X", &[("yy", 47)]),
    ("U", &[("St", 96)]),
];
pub const NUMBER_MAP: [(&str, &[(&str, i32); 1]); 10] = [
    ("0", &[("ab", 63)]),
    ("1", &[("cd", 18)]),
    ("2", &[("eF", 17)]),
    ("3", &[("gH", 28)]),
    ("4", &[("iJ", 37)]),
    ("5", &[("Kl", 72)]),
    ("6", &[("nm", 49)]),
    //
    ("7", &[("Hn", 92)]),
    ("8", &[("Pv", 65)]),
    ("9", &[("Nt", 89)]),
];
const SPECIAL_CHAR_MAP: [(&str, &[(&str, i32); 1]); 24] = [
    ("!", &[("gv", 66)]),
    ("@", &[("jk", 41)]),
    ("#", &[("Gt", 77)]),
    ("$", &[("fg", 78)]),
    ("%", &[("mm", 45)]),
    ("^", &[("db", 44)]),
    ("&", &[("pd", 39)]),
    //
    ("*", &[("Av", 67)]),
    ("(", &[("bQ", 14)]),
    (")", &[("TG", 24)]),
    ("_", &[("df", 22)]),
    ("+", &[("eH", 21)]),
    ("{", &[("Jp", 71)]),
    ("}", &[("Fk", 48)]),
    (":", &[("sr", 74)]),
    (";", &[("gM", 76)]),
    ("\"", &[("Io", 38)]),
    ("<", &[("aZ", 23)]),
    (">", &[("Lr", 53)]),
    ("?", &[("Ms", 27)]),
    ("/", &[("Kq", 16)]),
    ("|", &[("Qw", 19)]),
    ("]", &[("Ou", 43)]),
    ("}", &[("Yz", 79)]),
];
