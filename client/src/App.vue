<script setup lang="ts">
import { authenticate, logout, checkAuthentication, setup } from "./api";
import { navRoutes } from "./components/layout/navRoutes";

enum NavState {
  Open,
  Reduced,
  Collapsed,
}
const NavStateCount = 3;
let $navState = NavState.Open as NavState;
const $reducedDynamicWidth = 2.5 as number;

enum AuthState {
  Unauthenticated,
  MfaRequired,
  Authenticated,
}
let $authState = AuthState.Unauthenticated as AuthState;
let $loginDisabled = true;

const $username = "";
let $password = "";

const $mobileMedia = $(useMediaQuery("only screen and (max-width: 768px)"));
if ($mobileMedia) $navState = NavState.Collapsed;

function collapseNavIfMobile() {
  if ($mobileMedia && $navState === NavState.Open)
    // Give new page time to find initial left before transitioning
    setTimeout(() => ($navState = NavState.Collapsed), 0);
}

function toggleNavState() {
  $navState = ($navState + 1) % NavStateCount;
  if ($mobileMedia && $navState === NavState.Reduced) $navState++;
}

async function tryLogin() {
  $loginDisabled = true;
  const res = await authenticate($username, $password);
  $password = "";
  $loginDisabled = false;
  if (res.error != null) console.info("authentication error");
  else {
    // TODO Check for MFA here
    //authState = AuthState.Authenticated;
    checkAuth();
  }
}

async function tryLogout() {
  console.info("Logging out...");
  $authState = AuthState.Unauthenticated;
  logout();
}

function UnauthorizedCallback() {
  console.info("Unauthenticated");
  $authState = AuthState.Unauthenticated;
}

async function checkAuth() {
  console.info("Checking Auth State...");
  const res = await checkAuthentication();
  $authState = res.auth;
  $loginDisabled = false;
  if ($authState === AuthState.Authenticated) console.info("Already Authenticated ", $authState);
  else if (res.error == null) console.info("Unauthorized");
  else console.info("Check Authentication error", res.error);
}

onMounted(async () => {
  setup(UnauthorizedCallback);
  await checkAuth();
  setInterval(
    function () {
      if ($authState === AuthState.Authenticated && !document.hidden) checkAuth();
    }.bind(this),
    120000,
  );
});
</script>

<template>
  <div
    v-if="$authState === AuthState.Authenticated"
    :style="`--reduced-dynamic-width: ${$reducedDynamicWidth}rem;`"
    :class="{
      layout: 1,
      'nav-state-open': $navState === NavState.Open,
      'nav-state-collapsed': $navState === NavState.Collapsed,
      'nav-state-reduced': $navState === NavState.Reduced,
    }"
  >
    <button class="nav-head cl-secondary cl-force-dark" @click="toggleNavState">
      <i-mdi-hamburger-menu />
      <h1>nfSense</h1>
    </button>

    <Portal from="page-header" class="page-header pad gap" />

    <div class="nav-body cl-secondary cl-force-dark">
      <div>
        <div>
          <NavElements
            :routes="navRoutes"
            :click-handler="collapseNavIfMobile"
            @update:expanded-depth="(val) => ($reducedDynamicWidth = 2.5 + 0.5 * val)"
          />
        </div>
      </div>
      <div class="flex-row">
        <router-link class="button" to="/help"><i-material-symbols-help-outline /></router-link>
        <router-link class="button" to="/settings"><i-material-symbols-settings /></router-link>
        <button @click="tryLogout"><i-material-symbols-logout /></button>
      </div>
    </div>

    <router-view v-if="$authState === AuthState.Authenticated" v-slot="{ Component, route }">
      <Transition name="fade">
        <component :is="Component" :key="{ route }" class="page-content pad gap" />
      </Transition>
    </router-view>
  </div>

  <Transition name="fade">
    <div v-if="$authState === AuthState.Unauthenticated" class="login">
      <FocusTrap>
        <form
          :disabled="$loginDisabled"
          class="cl-secondary"
          @submit="($event) => $event.preventDefault()"
        >
          <h1>nfSense Login</h1>
          <h2 :hidden="!$loginDisabled">Logging in...</h2>
          <label for="username" :hidden="$loginDisabled" v-text="'Username'" />
          <input
            v-model="$username"
            name="username"
            :hidden="$loginDisabled"
            :disabled="$loginDisabled"
          />
          <label for="password" :hidden="$loginDisabled" v-text="'Password'" />
          <input
            v-model="$password"
            name="password"
            type="password"
            :hidden="$loginDisabled"
            :disabled="$loginDisabled"
          />
          <button @click="tryLogin">Login</button>
        </form>
      </FocusTrap>
    </div>
  </Transition>
</template>

<style>
/* Basic Layout */
.layout,
.login {
  position: absolute;
  left: 0px;
  right: 0px;
  top: 0px;
  bottom: 0px;

  display: grid;
}
.layout {
  grid-template-rows: auto 1fr;
  grid-template-columns: auto 1fr;
  grid-template-areas:
    "NH PH"
    "NB PC";
  --reduced-width: 3.5rem;

  &:not(.nav-state-open) {
    --reduced-width: var(--reduced-dynamic-width);
  }
}
.login {
  place-items: center;
}

.nav-head {
  grid-area: NH;

  font-weight: bold;
  text-align: center;

  &:focus {
    background: var(--cl-bg);
  }
  &:hover {
    background: var(--cl-bg-el);
  }
  & > h1 {
    flex-grow: 1;
  }
}
.nav-body {
  grid-area: NB;

  display: grid;
  grid-template: 1fr auto/ 1fr;

  & .button {
    justify-content: left;
  }
  & .flex-row * {
    flex: 1;
  }
  & > :first-child {
    scrollbar-width: none;
  }
  & .button:not(:hover) {
    background: transparent;
  }
  & > :first-child {
    background: /* Top/Bottom Cover, Top/Bottom Shadow */
      linear-gradient(var(--cl-bg) 30%, rgba(0, 0, 0, 0)) center top,
      linear-gradient(rgba(0, 0, 0, 0), var(--cl-bg) 70%) center bottom,
      linear-gradient(rgba(0, 0, 0, 0.5), rgba(0, 0, 0, 0)) center top,
      linear-gradient(rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.5)) center bottom;
    background-repeat: no-repeat;
    background-size:
      100% 40px,
      100% 40px,
      100% 14px,
      100% 14px;
    background-attachment: local, local, scroll, scroll;
    overflow-y: auto;

    & > * {
      display: grid;
      grid-template-columns: calc(var(--reduced-width) - 0.25rem) 1fr; /* -0.25rem adjustment is for halved 0.5rem padding */
      place-self: start;
      transition: grid-template-columns 0.2s ease-out;
      width: 100%;
    }
    &::-webkit-scrollbar {
      display: none;
    }
  }
  & svg {
    place-self: center;
  }
}
.page-header {
  grid-area: PH;

  flex-flow: row nowrap;
  align-items: center;

  & button svg {
    margin: -0.25rem;
  }
}
.page-content {
  grid-area: PC;
  overflow-y: auto;
}

.nav-body > :first-child > * > *,
.nav-dropdown > *,
.nav-dropdown > :first-child,
.nav-dropdown-body > * {
  grid-column: 1 / 3;
  display: grid;
  grid-template-columns: subgrid;
}

/* Nav-Body-Collapsing */
.nav-body,
.page-header,
.page-content {
  position: relative;
  left: 0%;
  width: 100%;
  transition:
    left 0.2s ease-out,
    width 0.2s ease-out;
}
.nav-state-reduced .nav-body {
  width: calc(0% + var(--reduced-width));
}
.nav-state-reduced .page-content {
  left: calc(calc(-100vw + 100%) + var(--reduced-width));
  width: calc(calc(0% + 100vw) - var(--reduced-width));
}
.nav-state-collapsed .nav-body {
  width: 0%;
}
.nav-state-collapsed .page-content {
  left: calc(-100vw + 100%);
  width: calc(0% + 100vw);
}
.layout:not(.nav-state-open) > .nav-body > .flex-row {
  flex-direction: column;
  align-items: start;
}

.nav-state-reduced > .nav-body > .flex-row > * {
  width: var(--reduced-width);
}

@media only screen and (min-width: 769px) {
  .nav-head > svg {
    display: none;
  }
}

/* Mobile Layout */
@media only screen and (max-width: 768px) {
  .layout {
    grid-template-rows: auto auto 1fr;
    grid-template-areas:
      "NH NH"
      "NB PH"
      "NB PC";
  }

  .nav-head > h1 {
    text-align: left;
  }

  .nav-state-collapsed .page-header {
    left: calc(-100vw + 100%);
    width: calc(0% + 100vw);
  }
  .nav-state-reduced .page-header {
    left: calc(calc(-100vw + 100%) + var(--reduced-width));
    width: calc(calc(0% + 100vw) - var(--reduced-width));
  }
  .nav-state-open .nav-body {
    width: calc(0% + 100vw);
  }
  .nav-state-open .page-content,
  .nav-state-open .page-header {
    left: 100%;
  }
}
</style>
