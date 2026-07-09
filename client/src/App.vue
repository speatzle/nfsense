<script setup lang="ts">
import { authenticate, logout, checkAuthentication, setup } from "./api";
import { navRoutes } from "./components/layout/navRoutes";

const toast = useToast();
const $mobileMedia = $(useMediaQuery("(max-width: 768px)"));
const { modalStack } = useModals();

let $navOpen = !$mobileMedia;
watch($$($mobileMedia), (x) => ($navOpen = !x));
let $navDepth = 0 as number;
function collapseNavIfMobile() {
  if ($mobileMedia && $navOpen)
    // Lets page find initial left to transition; nextTick will not work due to microtask behavior
    setTimeout(() => ($navOpen = false), 0);
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
    toast.error("Authentication failed");
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
    :style="`
      --reduced-width: ${$navOpen ? 3.5 : 2.5 + $navDepth * 0.5}rem;
      --sidepane-width: ${!$mobileMedia && modalStack.length ? 'min(40rem, 50vw)' : '0px'};
    `"
    :class="{ layout: 1, 'nav-open': $navOpen }"
  >
    <button class="nav-head" @click="$navOpen = !$navOpen">
      <i-mdi-letter-n-box />
      <h1>nfSense</h1>
    </button>
    <div class="nav-body">
      <div class="nav-container">
        <NavElements
          :routes="navRoutes"
          :click-handler="collapseNavIfMobile"
          @update:depth="(val) => ($navDepth = val)"
        />
      </div>
      <div class="flex-row">
        <router-link class="button" to="/help"><i-material-symbols-help-outline /></router-link>
        <router-link class="button" to="/settings"><i-material-symbols-settings /></router-link>
        <button @click="tryLogout"><i-material-symbols-logout /></button>
      </div>
    </div>

    <router-view v-if="$authState === AuthState.Authenticated" v-slot="{ Component, route }">
      <Transition name="fade">
        <component :is="Component" :key="route.fullPath" class="page-content pad gap" />
      </Transition>
    </router-view>

    <Popups v-if="$mobileMedia" />
    <Sidepane v-else />
  </div>

  <Transition name="fade">
    <div v-if="$authState === AuthState.Unauthenticated" class="login">
      <!-- <FocusTrap> -->
      <form class="pad" @submit.prevent>
        <h1>nfSense Login</h1>
        <h2 v-show="$loginDisabled">Logging in...</h2>
        <label for="username" :hidden="$loginDisabled" v-text="'Username'" />
        <input v-model="$username" name="username" v-show="!$loginDisabled" />
        <label for="password" :hidden="$loginDisabled" v-text="'Password'" />
        <input v-model="$password" name="password" type="password" v-show="!$loginDisabled" />
        <button @click="tryLogin" class="accent">Login</button>
      </form>
      <!-- </FocusTrap> -->
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
  grid-template-columns: auto 1fr var(--sidepane-width);
  transition: grid-template-columns 0.2s ease-out;
  grid-template-areas:
    "NH PC SP"
    "NB PC SP";
}
.login {
  place-items: center;
  & > form {
    --cl-z: 1;
    background: var(--cl-bg);
    & > button {
      --cl-z: 2;
    }
  }
}

:is(.nav-head, .nav-body) {
  --cl-z: 1;
  border-right-style: solid;
}

.nav-head {
  grid-area: NH;
  justify-content: left;
  padding-left: 0px;
  gap: 0px;
  --button-border: 0px;

  & > svg {
    --cl-fg-l: var(--cl-accent-fg-l);
    --cl-base: var(--cl-primary);
    background: radial-gradient(
      circle,
      var(--cl-fg) 0%,
      var(--cl-fg) 25%,
      #00000000 25%,
      #00000000 100%
    );
  }
  & > svg > * {
    --cl-fg-l: var(--cl-accent-bg-l);
    --cl-base: var(--cl-primary);
  }
  &:focus {
    --cl-i: 0;
  }
  &:hover {
    --cl-i: 1;
  }
  & > svg {
    width: var(--reduced-width);
    transition: width 0.2s ease-out;
    transform: scale(2);
  }
}
.nav-body {
  grid-area: NB;
  display: grid;
  grid-template: 1fr auto / 1fr;
  overflow: auto;
  background-color: var(--cl-bg);
  --button-border: 0px;

  & :is(button, .button):not(:hover, :focus) {
    background: transparent;
  }

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
    & > .nav-elements {
      grid-template-columns:
        calc(var(--reduced-width) - 0.25rem)
        1fr auto; /* -0.25rem adjustment is for halved 0.5rem padding */
    }
  }
}
.sidepane {
  grid-area: SP;
}

/* Nav-Body-Collapsing */
:is(.nav-head, .nav-body, .page) {
  position: relative; /* Allows individual offsets */
  left: 0%; /* Transition Baseline */
  width: 100%; /* Transition Baseline */
  transition: all 0.2s ease-out; /* all avoids interfering with page fade */
}

/* Desktop */
@media (min-width: 769px) {
  :is(.nav-head, .nav-body) {
    border-right-width: 2px;
  }
  .layout:not(.nav-open) :is(.nav-head, .nav-body) {
    width: calc(0% + var(--reduced-width) + 2px);
  }
  .layout:not(.nav-open) .page {
    left: calc(calc(-100vw + 100%) + var(--sidepane-width) + var(--reduced-width) + 2px);
    width: calc(calc(0% + 100vw) - var(--sidepane-width) - var(--reduced-width) - 2px);
  }
  .layout:not(.nav-open) > .nav-body > .flex-row {
    flex-direction: column;
    align-items: start;

    & > * {
      width: var(--reduced-width);
      transition: all 0.2s ease-out;
    }
  }
}

/* Mobile */
@media (max-width: 768px) {
  :is(.nav-head, .nav-body) {
    border-right-width: 0px;
  }
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
  .nav-body {
    width: 0px;
  }

  .layout:not(.nav-open) .page {
    left: calc(-100vw + 100%);
    width: calc(0% + 100vw);
  }
  .layout.nav-open .nav-body {
    width: calc(0% + 100vw);
  }
  .layout.nav-open .page {
    left: 100%;
  }
}
</style>
