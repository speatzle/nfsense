<script setup lang="ts">
import { authenticate, logout, checkAuthentication, setup } from './api';

// Icons https://icon-sets.iconify.design/?query=system
import IDashboard from '~icons/ri/dashboard-2-line';
import IRule from '~icons/material-symbols/rule-folder-outline-sharp';
import IAddress from '~icons/eos-icons/ip';
import IEthernet from '~icons/bi/ethernet';
import IService from '~icons/material-symbols/home-repair-service';
import ISNAT from '~icons/mdi/arrow-expand-right';
import IDNAT from '~icons/mdi/arrow-collapse-right';
import IConfig from '~icons/grommet-icons/document-config';
import IStaticRoutes from '~icons/material-symbols/drive-folder-upload-outline-sharp';
import IDNSServer from '~icons/carbon/server-dns';
import ITimeServer from '~icons/carbon/server-time';
import IWireguard from '~icons/simple-icons/wireguard';
import IDHCPServer from '~icons/material-symbols/book-rounded';
import IUser from '~icons/mdi/user';
import IServer from '~icons/ri/server-line';
import INodes from '~icons/fa6-solid/share-nodes';
import IList from '~icons/material-symbols/format-list-bulleted';
import IFirewall from '~icons/mdi/wall-fire';
import INetwork from '~icons/mdi/lan';
import IVPN from '~icons/mdi/vpn';
import IRouter from '~icons/mdi/router';
import ISettings from '~icons/material-symbols/settings';
import IShield from '~icons/mdi/shield-outline';



enum NavState { Open, Reduced, Collapsed };
const NavStateCount = 3;
let navState = $ref(NavState.Open);
const reducedDynamicWidth = $ref(2.5);

const navRoutesNew = [
  { caption: 'Dashboard', icon: IDashboard, href: '/' },
  { caption: 'Firewall', icon: IFirewall, children: [
    { caption: 'Rules', icon: IRule, href: '/firewall/forward_rules' },
    { caption: 'SNAT', icon: ISNAT, href: '/firewall/source_nat_rules' },
    { caption: 'DNAT', icon: IDNAT, href: '/firewall/destination_nat_rules' },
    { caption: 'Inbound', icon: IShield, href: '/firewall/inbound_rules' },
  ] },
  { caption: 'Network', icon: INetwork, children: [
    { caption: 'Interfaces', icon: IEthernet, href: '/network/interfaces' },
    { caption: 'Static Routes', icon: IStaticRoutes, href: '/network/static_routes' },
    { caption: 'Virtual Routers', icon: IRouter, href: '/network/virtual_routers' },
  ] },
  { caption: 'Objects', icon: IList, children: [
    { caption: 'Addresses', icon: IAddress, href: '/object/addresses' },
    { caption: 'Services', icon: IService, href: '/object/services' },
  ] },
  { caption: 'Services', icon: IServer, children: [
    { caption: 'DHCP', icon: IDHCPServer, href: '/service/dhcp_servers' },
    { caption: 'DNS', icon: IDNSServer, href: '/service/dns_servers' },
    { caption: 'NTP', icon: ITimeServer, href: '/service/ntp_servers' },
  ] },
  { caption: 'VPN', icon: IVPN, children: [
    { caption: 'Wireguard', icon: IWireguard, children: [
      { caption: 'Status', icon: IDashboard, href: '/vpn/wireguard_status' },
      { caption: 'Interfaces', icon: IEthernet, href: '/vpn/wireguard.interfaces' },
      { caption: 'Peers', icon: INodes, href: '/vpn/wireguard.peers' },
    ] },
  ] },
  { caption: 'Users', icon: IUser, href: '/system/users' },
  { caption: 'Config', icon: IConfig, href: '/config/config' },
];

enum AuthState { Unauthenticated, MfaRequired, Authenticated };
let authState = $ref(AuthState.Unauthenticated);
let loginDisabled = $ref(true);

const username = $ref('');
let password = $ref('');

const mobileMedia = window.matchMedia('only screen and (max-width: 768px)');
if (mobileMedia.matches) {
  navState = NavState.Collapsed;
}

function collapseNavIfMobile() {
  if (mobileMedia.matches && navState === NavState.Open) {
    // Give new page time to find initial left before transitioning
    setTimeout(() => navState = NavState.Collapsed, 0);
  }
}

function toggleNavState() {
  navState = (navState + 1) % NavStateCount;
  if (mobileMedia.matches && navState === NavState.Reduced)
    navState++;
}

async function tryLogin() {
  loginDisabled = true;
  const res = await authenticate(username, password);
  password = '';
  loginDisabled = false;
  if (res.error != null) {
    console.info('authentication error');
  }
  else {
    // TODO Check for MFA here
    authState = AuthState.Authenticated;
  }
}

async function tryLogout() {
  console.info('Logging out...');
  authState = AuthState.Unauthenticated;
  logout();
}

function UnauthorizedCallback() {
  console.info('Unauthenticated');
  authState = AuthState.Unauthenticated;
}

async function checkAuth() {
  console.info('Checking Auth State...');
  const res = await checkAuthentication();
  authState = res.auth;
  loginDisabled = false;
  if (authState === AuthState.Authenticated) {
    console.info('Already Authenticated ', authState);
  } else if (res.error == null) {
    console.info('Unauthorized');
  }
  else console.info('Check Authentication error',res.error);
}

onMounted(async() => {
  setup(UnauthorizedCallback);
  await checkAuth();
  setInterval(function () {
    if (authState === AuthState.Authenticated && !document.hidden) {
      checkAuth();
    }
  }.bind(this), 120000);
});

</script>

<template>
  <div v-if="authState === AuthState.Authenticated" :style="`--reduced-dynamic-width: ${reducedDynamicWidth}rem;`" :class="{
    'layout': 1,
    'nav-state-open': navState === NavState.Open,
    'nav-state-collapsed': navState === NavState.Collapsed,
    'nav-state-reduced': navState === NavState.Reduced,
  }">
    <button class="nav-head cl-secondary cl-force-dark" @click="toggleNavState">
      <i-mdi-hamburger-menu/>
      <h1>nfSense</h1>
    </button>

    <Portal from="page-header" class="page-header pad gap"/>

    <div class="nav-body cl-secondary cl-force-dark">
      <div>
        <div>
          <NavElements :routes="navRoutesNew" :click-handler="collapseNavIfMobile" @update:expanded-depth="(val) => reducedDynamicWidth = 2.5 + 0.5 * val"/>
        </div>
      </div>
      <div class="flex-row">
        <router-link class="button" to="/help"><i-material-symbols-help-outline/></router-link>
        <router-link class="button" to="/settings"><i-material-symbols-settings/></router-link>
        <button @click="tryLogout"><i-material-symbols-logout/></button>
      </div>
    </div>

    <router-view v-if="authState === AuthState.Authenticated" v-slot="{ Component, route }">
      <Transition name="fade">
        <component :is="Component" :key="{route}" class="page-content pad gap"/>
      </Transition>
    </router-view>
  </div>

  <Transition name="fade">
    <div v-if="authState === AuthState.Unauthenticated" class="login">
      <FocusTrap>
        <form :disabled="loginDisabled" class="cl-secondary" @submit="$event => $event.preventDefault()">
          <h1>nfSense Login</h1>
          <h2 :hidden="!loginDisabled">Logging in...</h2>
          <label for="username" :hidden="loginDisabled" v-text="'Username'" />
          <input v-model="username" name="username" :hidden="loginDisabled" :disabled="loginDisabled"/>
          <label for="password" :hidden="loginDisabled" v-text="'Password'"/>
          <input v-model="password" name="password" type="password" :hidden="loginDisabled" :disabled="loginDisabled"/>
          <button @click="tryLogin">Login</button>
        </form>
      </FocusTrap>
    </div>
  </Transition>
</template>

<style>
/* Basic Layout */
.layout, .login {
  position: absolute;
  left: 0px; right: 0px; top: 0px; bottom: 0px;

  display: grid;
}
.layout {
  grid-template-rows: auto 1fr;
  grid-template-columns: auto 1fr;
  grid-template-areas:
    "NH PH"
    "NB PC";
}
.layout:not(.nav-state-open) { --reduced-width: var(--reduced-dynamic-width); }
.nav-state-open { --reduced-width: 3.5rem; }
.login { place-items: center; }

.nav-head { grid-area: NH; }
.nav-body { grid-area: NB; }
.page-header { grid-area: PH; }
.page-content { grid-area: PC; }

.nav-head { font-weight: bold; text-align: center; }
.nav-head:focus { background: var(--cl-bg); }
.nav-head:hover { background: var(--cl-bg-el); }
.nav-head > svg { display: none; }
.nav-head > h1 { flex-grow: 1; }

.nav-body .button { justify-content: left; }
.nav-body .flex-row * { flex: 1; }

.nav-body {
  display: grid;
  grid-template: 1fr auto/ 1fr;
}

.nav-body > :first-child, .page-content { overflow-y: auto; }
.nav-body > :first-child::-webkit-scrollbar { display: none; }
.nav-body > :first-child { scrollbar-width: none; }

.nav-body .button:not(:hover) { background: transparent; }
.nav-body > :first-child {
  background:
    linear-gradient(var(--cl-bg) 30%, rgba(0, 0, 0, 0)) center top, /* Top Cover */
    linear-gradient(rgba(0, 0, 0, 0), var(--cl-bg) 70%) center bottom, /* Bottom Cover */
    linear-gradient(rgba(0, 0, 0, 0.5), rgba(0, 0, 0, 0)) center top, /* Top Shadow */
    linear-gradient(rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.5)) center bottom; /* Bottom Shadow */
  background-repeat: no-repeat;
  background-size: 100% 40px, 100% 40px, 100% 14px, 100% 14px;
  background-attachment: local, local, scroll, scroll;
}
.nav-body > :first-child > * {
  display: grid;
  grid-template-columns: calc(var(--reduced-width) - 0.25rem) 1fr; /* -0.25rem adjustment is for halved 0.5rem padding */
  place-self: start;
  transition: grid-template-columns 0.2s ease-out;
  width: 100%;
}
.nav-body > :first-child > * > *, .nav-dropdown > *, .nav-dropdown > :first-child, .nav-dropdown-body > * {
  grid-column: 1 / 3;
  display: grid;
  grid-template-columns: subgrid;
}

:is(.nav-body > :first-child > * > *, .nav-dropdown > *, .nav-dropdown > :first-child, .nav-dropdown-body > *) > svg {
  place-self: center;
}

/* Page */
.page-header {
  flex-flow: row nowrap;
  align-items: center;
}
.page-header button svg { margin: -0.25rem; }

/* Nav-Body-Collapsing */
.nav-body, .page-header, .page-content {
  position: relative;
  left: 0%;
  width: 100%;
  transition: left 0.2s ease-out, width 0.2s ease-out;
}
.nav-state-reduced .nav-body { width: calc(0% + var(--reduced-width)); }
.nav-state-reduced .page-content {
  left: calc(calc(-100vw + 100%) + var(--reduced-width));
  width: calc(calc(0% + 100vw) - var(--reduced-width));
}
.nav-state-collapsed .nav-body { width: 0%; }
.nav-state-collapsed .page-content {
  left: calc(-100vw + 100%);
  width: calc(0% + 100vw);
}
:not(.nav-state-open) > .nav-body > .flex-row {
  flex-direction: column;
  align-items: start;
}

.nav-state-reduced > .nav-body > .flex-row > * { width: var(--reduced-width); }

/* Mobile Layout */
@media only screen and (max-width: 768px) {
  .layout {
    grid-template-columns: auto 1fr;
    grid-template-rows: auto auto 1fr;
    grid-template-areas:
      "NH NH"
      "NB PH"
      "NB PC";
  }

  .nav-head > svg { display: initial; }
  .nav-head > h1 { text-align: left; }

  .nav-state-collapsed .page-header {
    left: calc(-100vw + 100%);
    width: calc(0% + 100vw);
  }
  .nav-state-reduced .page-header {
    left: calc(calc(-100vw + 100%) + var(--reduced-width));
    width: calc(calc(0% + 100vw) - var(--reduced-width));
  }
  .nav-state-open .nav-body { width: calc(0% + 100vw); }
  .nav-state-open .page-content,
  .nav-state-open .page-header {
    left: 100%;
  }
}
</style>
