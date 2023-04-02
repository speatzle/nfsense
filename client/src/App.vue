<script setup lang="ts">
import { authenticate, logout, checkAuthentication, setup } from "./api";

// Icons
import IDashboard from '~icons/ri/dashboard-2-line';
import IRule from '~icons/material-symbols/rule-folder-outline-sharp';
import IAddress from '~icons/eos-icons/ip';
import IEthernet from '~icons/bi/ethernet';
import IService from '~icons/material-symbols/home-repair-service';
import ISNAT from '~icons/mdi/arrow-expand-right';
import IDNAT from '~icons/mdi/arrow-collapse-right';
import IConfig from '~icons/grommet-icons/document-config';

enum NavState { Open, Reduced, Collapsed };
const NavStateCount = 3;
let navState = $ref(NavState.Open);
const navRoutes = {
  "/": { icon: IDashboard, caption: "Dashboard" },
  "/firewall/forwardrules": { icon: IRule, caption: "Rules" },
  "/firewall/sourcenatrules": { icon: ISNAT, caption: "SNAT" },
  "/firewall/destinationnatrules": { icon: IDNAT, caption: "DNAT" },
  "/network/interfaces": { icon: IEthernet, caption: "Interfaces" },
  "/object/addresses": { icon: IAddress, caption: "Addresses" },
  "/object/services": { icon: IService, caption: "Services" },
  "/config/config": { icon: IConfig, caption: "Config" },
};

enum AuthState { Unauthenticated, MfaRequired, Authenticated };
let authState = $ref(AuthState.Unauthenticated);
let loginDisabled = $ref(true);

let username = $ref("");
let password = $ref("");

const mobileMedia = window.matchMedia("only screen and (max-width: 768px)");
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
  password = "";
  loginDisabled = false;
  if (res.error != null) {
    console.info("authentication error");
  }
  else {
    // TODO Check for MFA here
    authState = AuthState.Authenticated;
  }
}

async function tryLogout() {
  console.info("Logging out...");
  authState = AuthState.Unauthenticated;
  logout();
}

function UnauthorizedCallback() {
  console.info("Unauthenticated");
  authState = AuthState.Unauthenticated;
}

async function checkAuth() {
  console.info("Checking Auth State...");
  let res = await checkAuthentication();
  authState = res.auth;
  loginDisabled = false;
  if (authState === AuthState.Authenticated) {
    console.info("Already Authenticated ", authState);
  } else if (res.error == null) {
    console.info("Unauthorized");
  }
  else console.info("Check Authentication error",res.error);
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
  <div v-if="authState === AuthState.Authenticated" :class="{
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
      <template v-for="(options, route) in navRoutes" :key="route">
        <router-link :to="route" class="button" @click="collapseNavIfMobile">
          <component :is="options.icon"/>
          {{ options.caption }}
        </router-link>
      </template>
      <div class="flex-grow"/>
      <div class="flex-row">
        <router-link class="button" to="/help"><i-material-symbols-help-outline/></router-link>
        <router-link class="button" to="/settings"><i-material-symbols-settings/></router-link>
        <button @click="tryLogout"><i-material-symbols-logout/></button>
      </div>
    </div>

    <router-view v-slot="{ Component, route }" v-if="authState === AuthState.Authenticated">
      <Transition name="fade">
        <component :is="Component" :key="{route}" class="page-content pad gap"/>
      </Transition>
    </router-view>
  </div>

  <Transition name="fade">
    <div class="login" v-if="authState === AuthState.Unauthenticated">
      <FocusTrap>
        <form @submit="$event => $event.preventDefault()" :disabled="loginDisabled" class="cl-secondary">
          <h1>nfSense Login</h1>
          <h2 :hidden="!loginDisabled">Logging in...</h2>
          <label for="username" v-text="'Username'" :hidden="loginDisabled" />
          <input name="username" v-model="username" :hidden="loginDisabled" :disabled="loginDisabled"/>
          <label for="password" v-text="'Password'" :hidden="loginDisabled"/>
          <input name="password" type="password" v-model="password" :hidden="loginDisabled" :disabled="loginDisabled"/>
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
.login { place-items: center; }

.nav-head { grid-area: NH; }
.nav-body { grid-area: NB; }
.page-header { grid-area: PH; }
.page-content { grid-area: PC; }

.nav-head { font-weight: bold; }
.nav-head > svg { display: none; }
.nav-head > h1 { flex-grow: 1; }

.nav-body .button { justify-content: left; }
.nav-body .flex-row * { flex: 1; }

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
  --reduced-width: 2.5rem;
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

  .nav-head > svg {
    display: initial;
  }

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