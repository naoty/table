package utils

import "golang.org/x/text/width"

// LenInHalfSize returns length of str in half size.
func LenInHalfSize(str string) int {
	len := 0

	for _, r := range str {
		p := width.LookupRune(r)
		switch p.Kind() {
		case width.EastAsianFullwidth, width.EastAsianWide:
			len += 2
		default:
			len++
		}
	}

	return len
}
