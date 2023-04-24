package dhcp

import (
	"embed"
	"fmt"
	"net"
	"net/netip"
	"strconv"
	"strings"
	"text/template"

	"nfsense.net/nfsense/internal/definitions/common"
	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/util"
)

//go:embed template
var templateFS embed.FS
var templates *template.Template

func init() {
	var err error
	templates, err = template.New("").Funcs(template.FuncMap{
		"getInterfaceAddress":          getInterfaceAddress,
		"getInterfaceNetworkAddress":   getInterfaceNetworkAddress,
		"getInterfaceBroadcastAddress": getInterfaceBroadcastAddress,
		"getInterfaceNetworkMask":      getInterfaceNetworkMask,
		"getAddressObjectsAsCommaList": getAddressObjectsAsCommaList,
		"getAddressObjectAsPoolRange":  getAddressObjectAsPoolRange,
		"getTimeInSecond":              getTimeInSecond,
	}).ParseFS(templateFS, "template/*.tmpl")
	if err != nil {
		panic(err)
	}
}

func getInterfaceAddress(conf config.Config, name string) string {
	return conf.Network.Interfaces[name].Address.Addr().String()
}

func getInterfaceNetworkAddress(conf config.Config, name string) string {
	return conf.Network.Interfaces[name].Address.Masked().Addr().String()
}

func getInterfaceBroadcastAddress(conf config.Config, name string) string {
	return util.BroadcastAddr(prefix2IPNet(*conf.Network.Interfaces[name].Address)).String()
}

func getInterfaceNetworkMask(conf config.Config, name string) string {
	return NetMaskToString(conf.Network.Interfaces[name].Address.Bits())
}

func getAddressObjectsAsCommaList(conf config.Config, names []string) string {
	res := ""
	for i, name := range names {
		res = res + conf.Object.Addresses[name].Host.String()
		if len(names)-1 != i {
			res = res + ", "
		}
	}
	return res
}

func getAddressObjectAsPoolRange(conf config.Config, name string) string {
	// TODO
	return strings.ReplaceAll(conf.Object.Addresses[name].Range.String(), "-", " ")
}

func getTimeInSecond(dur common.Duration) string {
	return fmt.Sprintf("%d", int(dur.Seconds()))
}

func prefix2IPNet(prefix netip.Prefix) net.IPNet {
	addr := prefix.Addr() // extract the address portion of the prefix
	pLen := 128           // plen is the total size of the subnet mask
	if addr.Is4() {
		pLen = 32
	}
	ones := prefix.Bits()            // ones is the portion of the mask that's set
	ip := net.IP(addr.AsSlice())     // convert the address portion to net.IP
	mask := net.CIDRMask(ones, pLen) // create a net.IPMask
	return net.IPNet{                // and construct the final IPNet
		IP:   ip,
		Mask: mask,
	}
}

func NetMaskToString(mask int) string {
	var binarystring string

	for ii := 1; ii <= mask; ii++ {
		binarystring = binarystring + "1"
	}
	for ii := 1; ii <= (32 - mask); ii++ {
		binarystring = binarystring + "0"
	}
	oct1 := binarystring[0:8]
	oct2 := binarystring[8:16]
	oct3 := binarystring[16:24]
	oct4 := binarystring[24:]

	ii1, _ := strconv.ParseInt(oct1, 2, 64)
	ii2, _ := strconv.ParseInt(oct2, 2, 64)
	ii3, _ := strconv.ParseInt(oct3, 2, 64)
	ii4, _ := strconv.ParseInt(oct4, 2, 64)
	return strconv.Itoa(int(ii1)) + "." + strconv.Itoa(int(ii2)) + "." + strconv.Itoa(int(ii3)) + "." + strconv.Itoa(int(ii4))
}
