package style

const (
	Reset = "\x1b[0m"
	Green = "\x1b[32m"
	Cyan  = "\x1b[36m"
)

func Color(code, text string) string {
	return code + text + Reset
}
