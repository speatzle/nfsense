import "./global-styles/atomic.css";
import "./global-styles/components.css";
import "./global-styles/colors.css";
import "./global-styles/mlfe.css";
import "./global-styles/transitions.css";
import "vue-toast-notification/dist/theme-default.css";

import NicerForm from "./components/input/NicerForm.vue";
import PillBar from "./components/input/PillBar.vue";
import TextBox from "./components/input/TextBox.vue";
import EnumInput from "./components/input/EnumInput.vue";
import NumberBox from "./components/input/NumberBox.vue";
import MultilineTextBox from "./components/input/MultilineTextBox.vue";
import CheckBox from "./components/input/CheckBox.vue";
import SingleSelect from "./components/input/SingleSelect.vue";
import MultiSelect from "./components/input/MultiSelect.vue";
import Heading from "./components/display/Heading.vue";

import App from "./App.vue";
import ToastPlugin from "vue-toast-notification";

const app = createApp(App);
import { router } from "~/router";

app.use(router);
app.use(ToastPlugin);

// Global Components
app.component("NicerForm", NicerForm);
app.component("PillBar", PillBar);
app.component("TextBox", TextBox);
app.component("NumberBox", NumberBox);
app.component("MultilineTextBox", MultilineTextBox);
app.component("CheckBox", CheckBox);
app.component("SingleSelect", SingleSelect);
app.component("MultiSelect", MultiSelect);
app.component("EnumInput", EnumInput);
app.component("Heading", Heading);

app.mount("#app");
