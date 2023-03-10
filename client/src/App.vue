<script setup lang="ts">
import IDashboard from '~icons/ri/dashboard-2-line';
import IRule from '~icons/material-symbols/rule-folder-outline-sharp';
import IAddress from '~icons/eos-icons/ip';

import { authenticate, logout, checkAuthentication, setup } from "./api";

enum NavState { Open, Reduced, Collapsed };
const NavStateCount = 3;
let navState = $ref(NavState.Open);
const navRoutes = {
  "/": { icon: IDashboard, caption: "Dashboard" },
  "/rules": { icon: IRule, caption: "Rules" },
  "/addresses": { icon: IAddress, caption: "Addresses" },
};

enum AuthState { Unauthenticated, MfaRequired, Authenticated };
let authState = $ref(AuthState.Unauthenticated);
let loginDisabled = $ref(true);

let username = $ref("");
let password = $ref("");


async function tryLogin() {
  loginDisabled = true;
  const res = await authenticate(username, password);
  password = "";
  loginDisabled = false;
  if (res.error != null) {
    console.info("authentication error");
  } else {
    // TODO Check for MFA here
    authState = AuthState.Authenticated;
  }
}

async function tryLogout() {
  logout();
  authState = AuthState.Unauthenticated;
}

function deAuthenticatedCallback() {
  console.info("Unauthenticated");
  authState = AuthState.Unauthenticated;
}

onMounted(async() => {
  setup(deAuthenticatedCallback);
  let res = await checkAuthentication();
  authState = res.auth;
  loginDisabled = false;
  if (authState === AuthState.Authenticated) {
    console.info("Already Authenticated ", authState);
  }
  else console.info("Check Authentication error",res.error);
});

</script>

<template>
  <div v-if="authState === AuthState.Authenticated" :class="{
    'layout': 1,
    'nav-state-open': navState === NavState.Open,
    'nav-state-collapsed': navState === NavState.Collapsed,
    'nav-state-reduced': navState === NavState.Reduced,
  }">
    <button class="nav-head" @click="() => navState = (navState + 1) % NavStateCount">
      nfSense
    </button>

    <Portal from="page-header" class="page-header pad gap"/>

    <div class="nav-body">
      <template v-for="(options, route) in navRoutes" :key="route">
        <router-link :to="route" class="button">
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
        <form @submit="$event => $event.preventDefault()" :disabled="loginDisabled">
          <h1>nfSense Login</h1>
          <label for="username" v-text="'Username'"/>
          <input name="username" v-model="username"/>
          <label for="password" v-text="'Password'" type="password"/>
          <input name="password" v-model="password"/>

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
  background-color: var(--cl-bg);
}
.layout {
  grid-template-rows: auto 1fr;
  grid-template-columns: auto 1fr;
}
.login { place-items: center; }

/* Navigation */
.nav-head, .nav-body { background: var(--cl-bg-low); }
.nav-head {
  font-size: 2rem;
  font-weight: bold;
}
.nav-body .button { justify-content: left; }
.nav-body .flex-row * { flex: 1; }

/* Page */
.page-header {
  grid-row: 1;
  grid-column: 2;
  flex-flow: row nowrap;
  align-items: center;
}
.page-header button svg {
  margin: -0.25rem;
}
.page-content {
  grid-row: 2;
  grid-column: 2;
  background: var(--cl-bg);
}

/* Nav-Body-Collapsing */
.nav-body, .page-content {
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
</style>