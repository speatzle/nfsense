export default function initiateCommonPlugins() {
  const router = useRouter();
  const toast = useToast();

  return { router, toast };
}