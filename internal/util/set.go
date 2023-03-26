package util

func ConvertSliceToSetString(slice []string) string {
	if len(slice) == 0 {
		return ""
	} else if len(slice) == 1 {
		return slice[0]
	}

	res := "{ "

	for i := range slice {
		res += " " + slice[i]
		if i < len(slice)-1 {
			res += ","
		}
	}

	res += " }"
	return res
}
