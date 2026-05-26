<script setup lang="ts">
import { authenticate, logout, checkAuthentication, setup } from "./api";
import { navRoutes } from "./components/layout/navRoutes";

const p = usePlugins();
const $mobileMedia = $(useMediaQuery("(max-width: 768px)"));

enum NavState {
  Open,
  Reduced,
  Collapsed,
}
const NavStateCount = 3;
let $navState = ($mobileMedia ? NavState.Collapsed : NavState.Open) as NavState;
watch($$($mobileMedia), (x) => ($navState = x ? NavState.Collapsed : NavState.Open));
let $minReducedWidth = 2.5 as number;
function collapseNavIfMobile() {
  if ($mobileMedia && $navState === NavState.Open)
    // Lets page find initial left to transition; nextTick will not work due to microtask behavior
    setTimeout(() => ($navState = NavState.Collapsed), 0);
}
function cycleNavState() {
  $navState = ($navState + 1) % NavStateCount;
  if ($mobileMedia && $navState === NavState.Reduced) $navState++;
}

enum AuthState {
  Unauthenticated,
  MfaRequired,
  Authenticated,
}
let $authState = AuthState.Unauthenticated as AuthState;
let $loginDisabled = true;
const $username = "";
let $password = "";

async function tryLogin() {
  $loginDisabled = true;
  const res = await authenticate($username, $password);
  $password = "";
  $loginDisabled = false;
  if (res.error != null) {
    console.info("authentication error");
    p.toast.error("Authentication failed");
  } else {
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

function unauthorizedCallback() {
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
  setup(unauthorizedCallback);
  await checkAuth();
  setInterval(() => {
    if ($authState === AuthState.Authenticated && !document.hidden) checkAuth();
  }, 120000);
});
</script>
<template>
  <div
    v-if="$authState === AuthState.Authenticated"
    :style="`--reduced-width: ${$navState === NavState.Open ? 3.5 : $minReducedWidth}rem;`"
    :class="{
      layout: 1,
      'nav-state-open': $navState === NavState.Open,
      'nav-state-collapsed': $navState === NavState.Collapsed,
      'nav-state-reduced': $navState === NavState.Reduced,
    }"
  >
    <button class="nav-head cl-secondary cl-force-dark" @click="cycleNavState">
      <i-mdi-hamburger-menu />
      <h1>nfSense</h1>
    </button>
    <div class="nav-body cl-secondary cl-force-dark">
      <div class="nav-container">
        <NavElements
          :routes="navRoutes"
          :click-handler="collapseNavIfMobile"
          @update:depth="(val) => ($minReducedWidth = 2.5 + 0.5 * val)"
        />
      </div>
      <div class="flex-row">
        <router-link class="button" to="/help"><i-material-symbols-help-outline /></router-link>
        <router-link class="button" to="/settings"><i-material-symbols-settings /></router-link>
        <button @click="tryLogout"><i-material-symbols-logout /></button>
      </div>
    </div>

    <Portal from="page-header" class="page-header pad gap" />
    <router-view v-if="$authState === AuthState.Authenticated" v-slot="{ Component, route }">
      <Transition name="fade">
        <component :is="Component" :key="route.fullPath" class="page-content pad gap" />
      </Transition>
    </router-view>
  </div>

  <Transition name="fade">
    <div v-if="$authState === AuthState.Unauthenticated" class="login">
      <FocusTrap>
        <form class="cl-secondary" @submit.prevent>
          <h1>nfSense Login</h1>
          <h2 v-show="$loginDisabled">Logging in...</h2>
          <label for="username" :hidden="$loginDisabled" v-text="'Username'" />
          <input v-model="$username" name="username" v-show="!$loginDisabled" />
          <label for="password" :hidden="$loginDisabled" v-text="'Password'" />
          <input v-model="$password" name="password" type="password" v-show="!$loginDisabled" />
          <button @click="tryLogin">Login</button>
        </form>
      </FocusTrap>
    </div>
  </Transition>
</template>
<style scoped>
/* Basic Layout */
:is(.layout, .login) {
  position: absolute;
  inset: 0px;
  display: grid;
}
.layout {
  grid-template-rows: auto 1fr;
  grid-template-columns: auto 1fr;
  grid-template-areas:
    "NH PH"
    "NB PC";
}
.login {
  place-items: center;
}

.nav-head {
  grid-area: NH;
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
  grid-template: 1fr auto / 1fr;

  & .flex-row * {
    flex: 1;
  }
  & .nav-container {
    scrollbar-width: none;
    background: /* Top/Bottom Cover, Top/Bottom Shadow */
      linear-gradient(var(--cl-bg) 30%, transparent) center top,
      linear-gradient(transparent, var(--cl-bg) 70%) center bottom,
      linear-gradient(#00000080, transparent) center top,
      linear-gradient(transparent, #00000080) center bottom;
    background-repeat: no-repeat;
    background-size:
      100% 40px,
      100% 40px,
      100% 14px,
      100% 14px;
    background-attachment: local, local, scroll, scroll;
    overflow-y: auto;

    &::-webkit-scrollbar {
      display: none;
    }
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

/* Nav-Body-Collapsing */
:is(.nav-body, .page-header, .page-content) {
  position: relative; /* Allows individual offsets */
  left: 0%; /* Transition Baseline */
  width: 100%; /* Transition Baseline */
  transition: all 0.2s ease-out; /* all avoids interfering with page fade */
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

@media (min-width: 769px) {
  .nav-head > svg {
    display: none;
  }
}

/* Mobile Layout */
@media (max-width: 768px) {
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
  .nav-state-open :is(.page-content, .page-header) {
    left: 100%;
  }
}
</style>
