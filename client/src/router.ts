import { createWebHistory, createRouter } from "vue-router";
import UpsertPage from "./components/pages/UpsertPage.vue";
import TablePage from "./components/pages/TablePage.vue";
import { subsystems } from "./definitions";
import ConfigPage from "./components/pages/ConfigPage.vue";
import LogsPage from "./components/pages/LogsPage.vue";
import UpdatesPage from "./components/pages/UpdatesPage.vue";
import WireguardStatusPage from "./components/pages/WireguardStatusPage.vue";
import WireguardInterfacesPage from "./components/pages/WireguardInterfacesPage.vue";
import DashboardPage from "./components/pages/DashboardPage.vue";

const tableViewRoutes = Object.entries(subsystems).map(([subsystem, sub]) => ({
  path: "/" + subsystem,
  children: Object.entries(sub.entities)
    .filter(([_, entity]) => entity.columns)
    .map(([entity, _]) => ({
      path: entity,
      component: TablePage,
      props: { subsystem, entity },
    })),
}));

// oxfmt-ignore
export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/:subsystem/:entity/:edit/:id?",
      component: UpsertPage,
      props: (p) => ({
        subsystem: p.params.subsystem,
        entity: p.params.entity,
        id: p.params.id === "" ? undefined : isNaN(+p.params.id) ? p.params.id : +p.params.id,
      }),
    },
    ...tableViewRoutes,
    { path: "/config/config", component: ConfigPage },
    { path: "/system/logs", component: LogsPage },
    { path: "/system/updates", component: UpdatesPage },
    { path: "/vpn/wireguard_status", component: WireguardStatusPage },
    { path: "/vpn/wireguard.interfaces", component: WireguardInterfacesPage },
    { path: "/", component: DashboardPage },
  ],
});
