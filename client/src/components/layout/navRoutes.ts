// Icons https://icon-sets.iconify.design/?query=system
import IDashboard from "~icons/ri/dashboard-2-line";
import IRule from "~icons/material-symbols/rule-folder-outline-sharp";
import IAddress from "~icons/eos-icons/ip";
import IEthernet from "~icons/bi/ethernet";
import IService from "~icons/material-symbols/home-repair-service";
import ISNAT from "~icons/mdi/arrow-expand-right";
import IDNAT from "~icons/mdi/arrow-collapse-right";
import IConfig from "~icons/grommet-icons/document-config";
import IStaticRoutes from "~icons/material-symbols/drive-folder-upload-outline-sharp";
import IDNSServer from "~icons/carbon/server-dns";
import ITimeServer from "~icons/carbon/server-time";
import IWireguard from "~icons/simple-icons/wireguard";
import IDHCPServer from "~icons/material-symbols/book-rounded";
import IUser from "~icons/mdi/user";
import IServer from "~icons/ri/server-line";
import INodes from "~icons/fa6-solid/share-nodes";
import IList from "~icons/material-symbols/format-list-bulleted";
import IFirewall from "~icons/mdi/wall-fire";
import INetwork from "~icons/mdi/lan";
import IVPN from "~icons/mdi/vpn";
import IRouter from "~icons/mdi/router";
import ISettings from "~icons/material-symbols/settings";
import IShield from "~icons/mdi/shield-outline";

// oxfmt-ignore
export const navRoutes = [
  { caption: "Dashboard", icon: IDashboard, href: "/" },
  { caption: "Firewall", icon: IFirewall, children: [
    { caption: "Rules", icon: IRule, href: "/firewall/forward_rules" },
    { caption: "SNAT", icon: ISNAT, href: "/firewall/source_nat_rules" },
    { caption: "DNAT", icon: IDNAT, href: "/firewall/destination_nat_rules" },
    { caption: "Inbound", icon: IShield, href: "/firewall/inbound_rules" },
  ] },
  { caption: "Network", icon: INetwork, children: [
    { caption: "Interfaces", icon: IEthernet, href: "/network/interfaces" },
    { caption: "Static Routes", icon: IStaticRoutes, href: "/network/static_routes" },
    { caption: "Policy Routes", icon: IStaticRoutes, href: "/network/policy_routes" },
    { caption: "Virtual Routers", icon: IRouter, href: "/network/virtual_routers" },
  ] },
  { caption: "Objects", icon: IList, children: [
    { caption: "Addresses", icon: IAddress, href: "/object/addresses" },
    { caption: "Services", icon: IService, href: "/object/services" },
  ] },
  { caption: "Services", icon: IServer, children: [
    { caption: "DHCP", icon: IDHCPServer, href: "/service/dhcp_servers" },
    { caption: "DNS", icon: IDNSServer, href: "/service/dns_servers" },
    { caption: "NTP", icon: ITimeServer, href: "/service/ntp_servers" },
  ] },
  { caption: "VPN", icon: IVPN, children: [
    { caption: "Wireguard", icon: IWireguard, children: [
      { caption: "Status", icon: IDashboard, href: "/vpn/wireguard_status" },
      { caption: "Interfaces", icon: IEthernet, href: "/vpn/wireguard.interfaces" },
      { caption: "Peers", icon: INodes, href: "/vpn/wireguard.peers" },
    ] },
  ] },
  { caption: "System", icon: ISettings, children: [
    { caption: "Users", icon: IUser, href: "/system/users" },
    { caption: "Log", icon: IService, href: "/system/logs" },
    { caption: "Updates", icon: IService, href: "/system/updates" },
  ] },
  { caption: "Config", icon: IConfig, href: "/config/config" },
];
