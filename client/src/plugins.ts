import { useRouter } from 'vue-router';
import { useToast } from "vue-toast-notification";

export default function initiateCommonPlugins() {
  const router = useRouter();
  const toast = useToast();

  return { router, toast };
}